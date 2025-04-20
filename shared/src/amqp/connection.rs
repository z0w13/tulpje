use amqprs::{
    channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use tokio::{select, sync::mpsc, time::sleep};
use tokio_util::sync::CancellationToken;

use crate::amqp::{AmqpChannelHandler, AmqpConnectionHandler, AmqpConsumer};

use super::{close_reason::CloseReason, event::Event, ConnectionArguments};

pub(crate) struct AmqpConnection {
    opts: ConnectionArguments,
    inner_opts: OpenConnectionArguments,

    recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,

    send_tx: mpsc::UnboundedSender<Vec<u8>>,
    send_rx: mpsc::UnboundedReceiver<Vec<u8>>,

    shutdown: CancellationToken,
}

impl AmqpConnection {
    pub(crate) fn new(
        inner_opts: OpenConnectionArguments,
        opts: ConnectionArguments,
        recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
        send_tx: mpsc::UnboundedSender<Vec<u8>>,
        send_rx: mpsc::UnboundedReceiver<Vec<u8>>,
        shutdown: CancellationToken,
    ) -> Self {
        Self {
            opts,
            inner_opts,

            recv_tx,

            send_tx,
            send_rx,

            shutdown,
        }
    }

    /// start the amqp connection
    pub(crate) async fn run(self) {
        if self.shutdown.is_cancelled() {
            tracing::warn!("shutdown was already called, not starting AmqpConnection");
            return;
        }

        let mut state = State::default();
        let mut shared = Shared {
            opts: self.opts,
            inner_opts: self.inner_opts,

            recv_tx: self.recv_tx,

            send_tx: self.send_tx,
            send_rx: self.send_rx,

            shutdown: self.shutdown,
        };

        loop {
            match state {
                State::Finished(reason) => {
                    tracing::info!("amqp connection finished: {reason}");
                    break;
                }
                _ => state = state.run(&mut shared).await,
            }
        }
    }
}

struct Shared {
    opts: ConnectionArguments,
    inner_opts: OpenConnectionArguments,

    recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,

    send_tx: mpsc::UnboundedSender<Vec<u8>>,
    send_rx: mpsc::UnboundedReceiver<Vec<u8>>,

    shutdown: CancellationToken,
}

enum State {
    /// base state, before starting, or when done running
    Disconnected(Disconnected),
    /// we're in the process of connecting for the first time
    Connecting(Connecting),
    /// the connection was closed for whatever reason, attempting to restore
    Reconnecting(Reconnecting),
    /// declaring the channel we're gonna be using
    OpeningChannel(OpeningChannel),
    /// the channel was closed for whatever reason, attempting to restore
    ReopeningChannel(ReopeningChannel),
    /// declaring a consumer for incoming messages
    DeclaringConsumer(DeclaringConsumer),
    /// connection is healthy, process events from amqp and send messages to amqp
    Connected(Connected),
    /// we're closing the channel because of server request, error or shutdown
    ClosingChannel(ClosingChannel),
    /// we're closing the connection because of server request, error or shutdown
    ClosingConnection(ClosingConnection),
    /// we've finished running
    Finished(CloseReason),
}

impl Default for State {
    fn default() -> Self {
        Self::Disconnected(Disconnected {})
    }
}

impl State {
    async fn run(self, shared: &mut Shared) -> Self {
        match self {
            Self::Disconnected(inner) => inner.run(shared).await,
            Self::Connecting(inner) => inner.run(shared).await,
            Self::Reconnecting(inner) => inner.run(shared).await,
            Self::OpeningChannel(inner) => inner.run(shared).await,
            Self::ReopeningChannel(inner) => inner.run(shared).await,
            Self::DeclaringConsumer(inner) => inner.run(shared).await,
            Self::Connected(inner) => inner.run(shared).await,
            Self::ClosingChannel(inner) => inner.run(shared).await,
            Self::ClosingConnection(inner) => inner.run(shared).await,
            Self::Finished(reason) => Self::Finished(reason),
        }
    }
}

struct Disconnected {}
impl Disconnected {
    fn connect(self) -> State {
        State::Connecting(Connecting {})
    }

    async fn run(self, _shared: &Shared) -> State {
        self.connect()
    }
}

struct Connecting {}
impl Connecting {
    fn open_channel(
        self,
        conn: Connection,
        event_tx: mpsc::UnboundedSender<Event>,
        event_rx: mpsc::UnboundedReceiver<Event>,
    ) -> State {
        State::OpeningChannel(OpeningChannel {
            conn,
            event_tx,
            event_rx,
        })
    }
    fn reconnect(self) -> State {
        State::Reconnecting(Reconnecting {})
    }
    fn close_connection(self, conn: Connection, reason: CloseReason) -> State {
        State::ClosingConnection(ClosingConnection::new(conn, reason))
    }

    async fn run(self, shared: &Shared) -> State {
        tracing::info!(
            "connecting to amqp at {}://{}:{}/{} ...",
            shared.inner_opts.get_scheme().unwrap_or("amqp"),
            shared.inner_opts.get_host(),
            shared.inner_opts.get_port(),
            shared.inner_opts.get_virtual_host(),
        );

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        tracing::debug!("opening connection to amqp ...");
        let Ok(conn) = Connection::open(&shared.inner_opts)
            .await
            .inspect_err(|err| tracing::error!("error connecting to amqp: {err}"))
        else {
            return self.reconnect();
        };

        tracing::debug!("registering connection callback handler ...");
        if let Err(err) = conn
            .register_callback(AmqpConnectionHandler::new(event_tx.clone()))
            .await
        {
            tracing::error!("failed to register amqp connection callback: {err}");
            return self.close_connection(conn, CloseReason::Other);
        };

        self.open_channel(conn, event_tx, event_rx)
    }
}

struct Reconnecting {}
impl Reconnecting {
    fn connect(self) -> State {
        State::Connecting(Connecting {})
    }

    async fn run(self, shared: &Shared) -> State {
        tracing::info!(
            "reconnecting in {}ms ...",
            shared.opts.reconnect_delay.as_millis(),
        );
        sleep(shared.opts.reconnect_delay).await;

        self.connect()
    }
}

pub(crate) struct OpeningChannel {
    conn: Connection,
    event_tx: mpsc::UnboundedSender<Event>,
    event_rx: mpsc::UnboundedReceiver<Event>,
}
impl OpeningChannel {
    fn connected(self, chan: Channel) -> State {
        State::Connected(Connected {
            conn: self.conn,
            chan,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        })
    }

    fn declare_consumer(self, chan: Channel) -> State {
        State::DeclaringConsumer(DeclaringConsumer {
            conn: self.conn,
            chan,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        })
    }

    fn reopen_channel(self) -> State {
        State::ReopeningChannel(ReopeningChannel {
            conn: self.conn,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        })
    }

    fn close_channel(self, chan: Channel, reason: CloseReason) -> State {
        State::ClosingChannel(ClosingChannel::new(
            self.conn,
            chan,
            self.event_tx,
            self.event_rx,
            reason,
        ))
    }

    async fn run(self, shared: &Shared) -> State {
        tracing::debug!("opening channel ...");
        let Ok(chan) = self
            .conn
            .open_channel(None)
            .await
            .inspect_err(|err| tracing::error!("couldn't create amqp channel: {err}"))
        else {
            return self.reopen_channel();
        };

        tracing::debug!("registering channel callback handler ...");
        if let Err(err) = chan
            .register_callback(AmqpChannelHandler::new(self.event_tx.clone()))
            .await
        {
            tracing::error!("failed to register amqp channel callback: {err}");
            return self.close_channel(chan, CloseReason::Other);
        }

        tracing::debug!("declaring queue '{}' ...", shared.opts.queue_name);
        if let Err(err) = chan
            .queue_declare(
                QueueDeclareArguments::new(&shared.opts.queue_name)
                    .durable(true)
                    .finish(),
            )
            .await
        {
            tracing::error!(
                "error declaring queue '{}': {}",
                shared.opts.queue_name,
                err
            );
            return self.close_channel(chan, CloseReason::Other);
        }

        if shared.recv_tx.is_some() {
            self.declare_consumer(chan)
        } else {
            self.connected(chan)
        }
    }
}

struct ReopeningChannel {
    conn: Connection,
    event_tx: mpsc::UnboundedSender<Event>,
    event_rx: mpsc::UnboundedReceiver<Event>,
}
impl ReopeningChannel {
    fn open_channel(self) -> State {
        State::OpeningChannel(OpeningChannel {
            conn: self.conn,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        })
    }

    async fn run(self, shared: &Shared) -> State {
        tracing::info!(
            "redeclaring channel in {}ms ...",
            shared.opts.reconnect_delay.as_millis(),
        );
        sleep(shared.opts.reconnect_delay).await;

        self.open_channel()
    }
}

struct DeclaringConsumer {
    conn: Connection,
    chan: Channel,
    event_tx: mpsc::UnboundedSender<Event>,
    event_rx: mpsc::UnboundedReceiver<Event>,
}
impl DeclaringConsumer {
    fn connected(self) -> State {
        State::Connected(Connected {
            conn: self.conn,
            chan: self.chan,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        })
    }

    fn close_channel(self, reason: CloseReason) -> State {
        State::ClosingChannel(ClosingChannel::new(
            self.conn,
            self.chan,
            self.event_tx,
            self.event_rx,
            reason,
        ))
    }

    async fn run(self, shared: &Shared) -> State {
        tracing::debug!("declaring amqp consumer ...");
        if let Err(err) = self
            .chan
            .basic_consume(
                AmqpConsumer::new(self.event_tx.clone()),
                BasicConsumeArguments::new(&shared.opts.queue_name, "")
                    .manual_ack(false)
                    .finish(),
            )
            .await
        {
            tracing::error!("error declaring consumer: {err}");
            return self.close_channel(CloseReason::Other);
        }

        self.connected()
    }
}

struct Connected {
    conn: Connection,
    chan: Channel,
    event_tx: mpsc::UnboundedSender<Event>,
    event_rx: mpsc::UnboundedReceiver<Event>,
}
impl Connected {
    fn close_channel(self, reason: CloseReason) -> State {
        State::ClosingChannel(ClosingChannel::new(
            self.conn,
            self.chan,
            self.event_tx,
            self.event_rx,
            reason,
        ))
    }

    async fn run(mut self, shared: &mut Shared) -> State {
        loop {
            select! {
                () = shared.shutdown.cancelled() => {
                    return self.close_channel(CloseReason::Shutdown);
                },
                event = self.event_rx.recv() => {
                    let Some(event) = event else {
                        tracing::warn!("event_rx was closed, reconnecting ...");
                        return self.close_channel(CloseReason::Fatal);
                    };
                    match event {
                        Event::ConnectionClose(_, _) => {
                            return self.close_channel(CloseReason::ConnectionClosed);
                        }
                        Event::ChannelClose(_, _) => {
                            return self.close_channel(CloseReason::ChannelClosed);
                        }
                        Event::ChannelPublishReturn(_chan, ret, _basic_properties, data) => {
                            if ret.reply_code() == 312 {
                                tracing::warn!("couldn't route message, triggering reconnect and requeuing message");
                                if let Err(err) = shared.send_tx.send(data) {
                                    tracing::error!("failed to requeue message: {err}");
                                };
                                return self.close_channel(CloseReason::PublishNoRoute);
                            }
                        }
                        Event::MessageReceived(_chan, _deliver, _basic_properties, data) => {
                            if let Some(recv_tx) = &shared.recv_tx {
                                if let Err(err) = recv_tx.send(data) {
                                    tracing::warn!("error sending received message to library user: {err}");
                                }
                            }
                        }
                        _ => {}
                    }
                }
                message = shared.send_rx.recv() => {
                    let Some(message) = message else {
                        tracing::error!("send_rx was closed, this shouldn't really happen ...");
                        return self.close_channel(CloseReason::Fatal);
                    };

                    if let Err(err) = self.chan.basic_publish(
                        BasicProperties::default(),
                        message,
                        // We set the mandatory flag so we can handle when the
                        // message can't be delivered to any queues, and then assume
                        // our queue was deleted and force a reconnect in
                        // ChannelCallback::publish_return
                        BasicPublishArguments::new("", &shared.opts.queue_name).mandatory(true).finish(),
                    )
                    .await {
                        tracing::error!("error sending event to amqp: {}", err);
                    }
                }
            }
        }
    }
}

struct ClosingChannel {
    conn: Connection,
    chan: Option<Channel>,
    event_tx: mpsc::UnboundedSender<Event>,
    event_rx: mpsc::UnboundedReceiver<Event>,
    reason: CloseReason,
}
impl ClosingChannel {
    fn new(
        conn: Connection,
        chan: Channel,
        event_tx: mpsc::UnboundedSender<Event>,
        event_rx: mpsc::UnboundedReceiver<Event>,
        reason: CloseReason,
    ) -> Self {
        Self {
            conn,
            chan: Some(chan),
            event_tx,
            event_rx,
            reason,
        }
    }

    fn close_connection(self) -> State {
        State::ClosingConnection(ClosingConnection::new(self.conn, self.reason))
    }

    fn reopen_channel(self) -> State {
        State::ReopeningChannel(ReopeningChannel {
            conn: self.conn,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        })
    }

    async fn run(mut self, _shared: &Shared) -> State {
        assert!(
            self.chan.is_some(),
            "ClosingChannel::chan is None, shouldn't happen, did you not use ClosingChan::new?"
        );

        // NOTE: Can unwrap safely, we consume self, and ClosingChannel::new enforces Some(chan)
        if let Err(err) = self.chan.take().unwrap().close().await {
            tracing::warn!("error closing amqp chan: {err}");
            return self.close_connection();
        }

        match self.reason {
            CloseReason::ChannelClosed | CloseReason::PublishNoRoute => self.reopen_channel(),
            CloseReason::ConnectionClosed
            | CloseReason::Fatal
            | CloseReason::Other
            | CloseReason::Shutdown => self.close_connection(),
        }
    }
}

struct ClosingConnection {
    conn: Option<Connection>,
    reason: CloseReason,
}
impl ClosingConnection {
    fn new(conn: Connection, reason: CloseReason) -> Self {
        Self {
            conn: Some(conn),
            reason,
        }
    }

    fn disconnect(self) -> State {
        State::Disconnected(Disconnected {})
    }
    fn reconnect(self) -> State {
        State::Reconnecting(Reconnecting {})
    }

    async fn run(mut self, _shared: &Shared) -> State {
        assert!(
            self.conn.is_some(),
            "ClosingChannel::chan is None, shouldn't happen, did you not use ClosingChan::new?"
        );

        // NOTE: Can unwrap safely, we consume self, and ClosingChannel::new enforces Some(chan)
        if let Err(err) = self.conn.take().unwrap().close().await {
            tracing::warn!("error closing amqp conn: {err}");
        }

        match self.reason {
            CloseReason::ChannelClosed
            | CloseReason::PublishNoRoute
            | CloseReason::ConnectionClosed
            | CloseReason::Other => self.reconnect(),
            CloseReason::Fatal | CloseReason::Shutdown => self.disconnect(),
        }
    }
}
