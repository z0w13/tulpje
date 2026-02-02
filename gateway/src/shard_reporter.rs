use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

use redis::{AsyncCommands as _, aio::ConnectionManager as RedisConnectionManager};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use twilight_gateway::{Event, EventType, EventTypeFlags, Latency};

use tulpje_shared::shard_state::ShardState;
use twilight_model::gateway::payload::incoming::{GuildCreate, GuildDelete, Hello, Ready};

pub(crate) const SHARD_REPORTER_EVENTS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
    EventTypeFlags::GATEWAY_HEARTBEAT_ACK.bits()
        | EventTypeFlags::GATEWAY_HELLO.bits()
        | EventTypeFlags::GUILD_CREATE.bits()
        | EventTypeFlags::GUILD_DELETE.bits()
        | EventTypeFlags::READY.bits()
        | EventTypeFlags::RESUMED.bits(),
);

#[derive(Debug)]
pub(crate) enum ReporterEvent {
    Event(Event),
    GatewayHeartbeatAck(Event, Latency),
}

impl ReporterEvent {
    pub(crate) fn from_event(event: Event, latency: &Latency) -> Self {
        match event {
            Event::GatewayHeartbeatAck => {
                ReporterEvent::GatewayHeartbeatAck(event, latency.clone())
            }
            evt => ReporterEvent::Event(evt),
        }
    }
    fn kind(&self) -> EventType {
        match self {
            Self::Event(evt) | Self::GatewayHeartbeatAck(evt, _) => evt.kind(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ShardReporterHandle {
    sender: mpsc::Sender<ReporterEvent>,
    shutdown: CancellationToken,
}
impl ShardReporterHandle {
    pub(crate) fn new(redis: RedisConnectionManager, shard_id: u32) -> (JoinHandle<()>, Self) {
        // TODO: Configure channel size?
        let (sender, receiver) = mpsc::channel(10);
        let shutdown = CancellationToken::new();

        let mut reporter = ShardReporter::new(redis, shard_id, receiver, shutdown.clone());
        let handle = tokio::spawn(async move { reporter.run().await });

        (handle, Self { sender, shutdown })
    }

    pub(crate) fn try_send(
        &self,
        msg: ReporterEvent,
    ) -> Result<(), Box<mpsc::error::TrySendError<ReporterEvent>>> {
        if SHARD_REPORTER_EVENTS.contains(msg.kind().into()) {
            Ok(self.sender.try_send(msg)?)
        } else {
            Ok(())
        }
    }

    pub(crate) fn shutdown(&mut self) {
        self.shutdown.cancel();
    }
}

pub struct ShardReporter {
    redis: RedisConnectionManager,
    guild_ids: HashSet<u64>,
    shard: ShardState,
    receiver: mpsc::Receiver<ReporterEvent>,
    shutdown: CancellationToken,
}

impl ShardReporter {
    pub fn new(
        redis: RedisConnectionManager,
        shard_id: u32,
        receiver: mpsc::Receiver<ReporterEvent>,
        shutdown: CancellationToken,
    ) -> Self {
        Self {
            redis,
            guild_ids: HashSet::new(),
            shard: ShardState::new(shard_id),
            receiver,
            shutdown,
        }
    }

    async fn run(&mut self) {
        tracing::info!("ShardReporter started...");
        loop {
            tokio::select! {
                msg = self.receiver.recv() => {
                    let Some(evt) = &msg else {
                        break;
                    };

                    if let Err(err) = self.handle_event(evt).await {
                        tracing::warn!(?evt, ?err, "error handling event")
                    };
                },
                () = self.shutdown.cancelled() => self.receiver.close(),
            }
        }
        tracing::info!("ShardReporter stopped...");
    }

    async fn handle_event(
        &mut self,
        evt: &ReporterEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match evt {
            ReporterEvent::Event(evt) => match evt {
                Event::GatewayHello(hello) => self.helloed(hello).await,
                Event::Ready(info) => self.readied(info).await,
                Event::GuildCreate(created) => self.guild_created(created).await,
                Event::GuildDelete(deleted) => self.guild_deleted(deleted).await,
                Event::Resumed => self.resumed().await,
                Event::GatewayClose(_) => self.socket_closed().await,
                _ => Ok(()),
            },
            ReporterEvent::GatewayHeartbeatAck(_evt, latency) => self.heartbeated(latency).await,
        }
    }

    async fn save_shard(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.redis
            .clone()
            .hset::<&str, String, &ShardState, ()>(
                "tulpje:shard_status",
                self.shard.shard_id.to_string(),
                &self.shard,
            )
            .await
            .map_err(|err| err.into())
    }

    async fn helloed(&mut self, hello: &Hello) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!(
            "shard {} hello received (heartbeat interval: {}ms)",
            self.shard.shard_id,
            hello.heartbeat_interval,
        );

        // heartbeat_interval is a u64, but should be within bounds of u32,
        // do error if it isn't for some reason
        self.shard.heartbeat_interval = u32::try_from(hello.heartbeat_interval)?;
        self.shard.last_connection = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs();

        self.save_shard().await
    }

    async fn readied(&mut self, ready: &Ready) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!(
            "shard {} ready ({} guilds)",
            self.shard.shard_id,
            ready.guilds.len()
        );

        self.guild_ids
            .extend(ready.guilds.iter().map(|g| g.id.get()));

        self.shard.up = true;
        self.shard.guild_count = self
            .guild_ids
            .len()
            .try_into()
            .expect("couldn't convert len() to u64");

        self.save_shard().await
    }

    async fn resumed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("shard {} resumed", self.shard.shard_id,);

        self.shard.up = true;
        self.shard.last_connection = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs();

        self.save_shard().await
    }

    async fn guild_created(
        &mut self,
        created: &GuildCreate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.guild_ids.insert(created.id().get()) {
            // guild was already in set, do nothing
            return Ok(());
        }

        self.shard.guild_count = self
            .guild_ids
            .len()
            .try_into()
            .expect("couldn't convert len() to u64");

        self.save_shard().await
    }

    async fn guild_deleted(
        &mut self,
        deleted: &GuildDelete,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.guild_ids.remove(&deleted.id.get()) {
            // guild wasn't in set, do nothing
            return Ok(());
        }

        self.shard.guild_count = self
            .guild_ids
            .len()
            .try_into()
            .expect("couldn't convert len() to u64");

        self.save_shard().await
    }

    async fn socket_closed(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("shard {} closed", self.shard.shard_id);

        self.shard.up = false;
        self.shard.disconnect_count += 1;

        self.save_shard().await
    }

    async fn heartbeated(&mut self, latency: &Latency) -> Result<(), Box<dyn std::error::Error>> {
        self.shard.up = true;
        self.shard.last_heartbeat = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs();

        self.shard.latency = latency
            .recent()
            .first()
            .expect("no latency measurement after heartbeat")
            .as_millis()
            .try_into()
            .expect("couldn't convert into u64");

        self.save_shard().await
    }
}
