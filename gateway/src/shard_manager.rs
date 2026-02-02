use futures_util::StreamExt as _;
use std::error::Error;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
use tulpje_shared::DiscordEvent;
use twilight_gateway::{CloseFrame, Message, Shard};

use crate::{
    metrics,
    parsed_event::ParsedEvent,
    shard_reporter::{ReporterEvent, ShardReporterHandle},
};

pub(crate) enum ShardManagerMessage {
    Start,
}

#[derive(Clone)]
pub(crate) struct ShardManagerHandle {
    sender: mpsc::UnboundedSender<ShardManagerMessage>,
    shutdown: CancellationToken,
}

#[derive(PartialEq)]
enum ShardState {
    Running,
    Stopping,
    Stopped,
}

impl ShardManagerHandle {
    pub(crate) fn new(
        shard: Shard,
        amqp_tx: UnboundedSender<Vec<u8>>,
        reporter: ShardReporterHandle,
    ) -> (JoinHandle<()>, Self) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let shutdown = CancellationToken::new();

        let mut shard_mgr = ShardManager::new(receiver, shard, amqp_tx, reporter, shutdown.clone());
        let handle = tokio::spawn(async move { shard_mgr.run().await });

        (handle, Self { sender, shutdown })
    }

    pub(crate) fn start(&self) -> Result<(), mpsc::error::SendError<ShardManagerMessage>> {
        self.sender.send(ShardManagerMessage::Start)
    }

    pub(crate) fn shutdown(&mut self) {
        self.shutdown.cancel();
    }
}

pub(crate) struct ShardManager {
    receiver: UnboundedReceiver<ShardManagerMessage>,
    shard: Shard,
    amqp_tx: UnboundedSender<Vec<u8>>,
    reporter: ShardReporterHandle,
    shutdown: CancellationToken,
    state: ShardState,
}

impl ShardManager {
    fn new(
        receiver: mpsc::UnboundedReceiver<ShardManagerMessage>,
        shard: Shard,
        amqp_tx: UnboundedSender<Vec<u8>>,
        reporter: ShardReporterHandle,
        shutdown: CancellationToken,
    ) -> Self {
        Self {
            receiver,
            shard,
            amqp_tx,
            reporter,
            shutdown,
            state: ShardState::Stopped,
        }
    }

    async fn run(&mut self) {
        tracing::info!("ShardManager started...");

        loop {
            tokio::select! {
                Some(msg) = self.receiver.recv() => {
                    match msg {
                        ShardManagerMessage::Start => {
                            self.state = ShardState::Running;
                            tracing::info!("Shard started...");
                        },
                    };
                },
                msg = self.shard.next(), if self.state != ShardState::Stopped => {
                    match msg {
                        Some(Ok(message)) => {
                            match self.handle_message(message) {
                                Ok(true) => { break },
                                Err(err) => tracing::warn!(?err, "error handling message"),
                                _ => ()
                            }
                        }
                        Some(Err(err)) => {
                            tracing::error!(?err, "error receiving discord message");
                        }
                        None => {
                            tracing::error!("empty message, connection irrecoverably closed, exiting...");
                            break;
                        }
                    }
                },
                () = self.shutdown.cancelled(), if self.state == ShardState::Running => {
                    tracing::info!("disconnecting from Discord...");
                    self.shard.close(CloseFrame::RESUME);
                    self.state = ShardState::Stopping;
                },
                else => break
            }
        }

        tracing::info!("ShardManager stopped...");
    }

    fn handle_message(&mut self, message: Message) -> Result<bool, Box<dyn Error>> {
        let event = ParsedEvent::from_message(message)
            .map_err(|err| format!("error parsing gateway message: {err}"))?;

        // if this is a close frame and we're shutting, we break at the
        // end of the loop, checking it here to avoid having to clone `event`
        let should_stop = event.is_close() && self.state == ShardState::Stopping;

        // track event metrics
        metrics::track_gateway_event(
            self.shard.id().number(),
            event.name.as_deref().unwrap_or("default"),
        );

        if let Some(event) = event.event {
            self.reporter
                .try_send(ReporterEvent::from_event(event, self.shard.latency()))
                .map_err(|err| format!("error sending message to ShardManager: {err}"))?;
        }

        if let Some(text) = event.text
            && event.forward
        {
            let event = DiscordEvent::new(self.shard.id().number(), text);

            let serialized_event = serde_json::to_vec(&event)
                .map_err(|err| format!("error serializing event: {err}"))?;

            self.amqp_tx
                .send(serialized_event)
                .map_err(|err| format!("error sending event to amqp: {err}"))?;

            tracing::debug!(
                uuid = ?event.meta.uuid,
                shard = event.meta.shard,
                "event sent"
            );
        }

        Ok(should_stop)
    }
}
