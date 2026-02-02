use futures_util::StreamExt as _;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;
use tulpje_shared::DiscordEvent;
use twilight_gateway::{CloseFrame, Shard};

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
enum ShardManagerState {
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
        }
    }

    async fn run(&mut self) {
        tracing::info!("ShardManager started...");
        let mut state = ShardManagerState::Stopped;

        loop {
            tokio::select! {
                Some(msg) = self.receiver.recv() => {
                    match msg {
                        ShardManagerMessage::Start => {
                            state = ShardManagerState::Running;
                            tracing::info!("Shard started...");
                        },
                    };
                },
                msg = self.shard.next(), if state != ShardManagerState::Stopped => {
                    match msg {
                        Some(Ok(message)) => {
                            let event = match ParsedEvent::from_message(message) {
                                Ok(evt) => evt,
                                Err(err) => {
                                    tracing::warn!("error parsing gateway message: {err}");
                                    continue;
                                }
                            };

                            // if this is a close frame and we're shutting, we break at the
                            // end of the loop, checking it here to avoid having to clone `event`
                            let should_stop = event.is_close() && state == ShardManagerState::Stopping;

                            // track event metrics
                            metrics::track_gateway_event(
                                self.shard.id().number(),
                                event.name.as_deref().unwrap_or("default"),
                            );

                            if let Some(event) = event.event
                                && let Err(err) = self.reporter
                                    .try_send(ReporterEvent::from_event(event, self.shard.latency()))
                            {
                                tracing::error!("error sending message to ShardManager: {err}");
                            }

                            if let Some(text) = event.text
                                && event.forward
                            {
                                let event = DiscordEvent::new(self.shard.id().number(), text);
                                let serialized_event = match serde_json::to_vec(&event) {
                                    Ok(val) => val,
                                    Err(err) => {
                                        tracing::error!("error serializing event: {}", err);
                                        continue;
                                    }
                                };

                                if let Err(err) = self.amqp_tx.send(serialized_event) {
                                    tracing::error!("error sending event to amqp: {}", err);
                                    continue;
                                }

                                tracing::debug!(
                                    uuid = ?event.meta.uuid,
                                    shard = event.meta.shard,
                                    "event sent"
                                );
                            }

                            if should_stop {
                                break
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
                () = self.shutdown.cancelled(), if state == ShardManagerState::Running => {
                    tracing::info!("disconnecting from Discord...");
                    self.shard.close(CloseFrame::RESUME);
                    state = ShardManagerState::Stopping;
                },
                else => break
            }
        }

        tracing::info!("ShardManager stopped...");
    }
}
