use amqprs::{
    channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use tokio::{
    select,
    sync::{mpsc, oneshot},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

use crate::{state_transition, AmqpChannelHandler, AmqpConnectionHandler, AmqpConsumer};

use super::{
    close_reason::CloseReason, event::Event, state_machine::IntoState as _, ConnectionArguments,
    Error,
};

pub(crate) struct AmqpConnection {
    opts: ConnectionArguments,
    inner_opts: OpenConnectionArguments,

    recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,

    send_tx: mpsc::UnboundedSender<Vec<u8>>,
    send_rx: mpsc::UnboundedReceiver<Vec<u8>>,

    start: CancellationToken,
    start_tx: oneshot::Sender<Option<Error>>,

    shutdown: CancellationToken,
}

impl AmqpConnection {
    #[expect(clippy::too_many_arguments, reason = "well we need these values")]
    pub(crate) fn new(
        opts: ConnectionArguments,
        inner_opts: OpenConnectionArguments,

        recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,

        send_tx: mpsc::UnboundedSender<Vec<u8>>,
        send_rx: mpsc::UnboundedReceiver<Vec<u8>>,

        start: CancellationToken,
        start_tx: oneshot::Sender<Option<Error>>,

        shutdown: CancellationToken,
    ) -> Self {
        Self {
            opts,
            inner_opts,

            recv_tx,

            send_tx,
            send_rx,

            start,
            start_tx,

            shutdown,
        }
    }

    /// start the amqp connection
    pub(crate) async fn run(self) {
        self.start.cancelled().await;

        if self.shutdown.is_cancelled() {
            tracing::warn!("shutdown was already called, not starting AmqpConnection");
            return;
        }

        let mut shared = AmqpSharedData {
            opts: self.opts,
            inner_opts: self.inner_opts,

            recv_tx: self.recv_tx,

            send_tx: self.send_tx,
            send_rx: self.send_rx,

            start_tx: Some(self.start_tx),
            shutdown: self.shutdown,
        };
        let mut state = State::Disconnected(Disconnected {});

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

struct AmqpSharedData {
    opts: ConnectionArguments,
    inner_opts: OpenConnectionArguments,

    recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,

    send_tx: mpsc::UnboundedSender<Vec<u8>>,
    send_rx: mpsc::UnboundedReceiver<Vec<u8>>,

    start_tx: Option<oneshot::Sender<Option<Error>>>,
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
    /// connection has been closed, from here we either finish or go back to reconnecting
    ClosedConnection(ClosedConnection),
    /// we've finished running
    Finished(CloseReason),
}

impl State {
    async fn run(self, shared: &mut AmqpSharedData) -> Self {
        match self {
            Self::Disconnected(inner) => inner.run().await,
            Self::Connecting(inner) => inner.run(shared).await,
            Self::Reconnecting(inner) => inner.run(shared).await,
            Self::OpeningChannel(inner) => inner.run(shared).await,
            Self::ReopeningChannel(inner) => inner.run(shared).await,
            Self::DeclaringConsumer(inner) => inner.run(shared).await,
            Self::Connected(inner) => inner.run(shared).await,
            Self::ClosingChannel(inner) => inner.run().await,
            Self::ClosingConnection(inner) => inner.run().await,
            Self::ClosedConnection(inner) => inner.run(shared).await,
            Self::Finished(reason) => Self::Finished(reason),
        }
    }
}

// Define state transitions
state_transition!(Disconnected => Connecting);
state_transition!(Reconnecting => Connecting);
state_transition!(ClosedConnection => [Reconnecting]);
state_transition!(ClosingChannel => [
    ReopeningChannel,
    ClosingConnection
]);
state_transition!(ClosingConnection => ClosedConnection);
state_transition!(Connecting => [
        OpeningChannel,
        ClosingConnection,
        ClosedConnection,
        Reconnecting
]);
state_transition!(Connected => ClosingChannel);
state_transition!(DeclaringConsumer => [Connected, ClosingChannel]);
state_transition!(OpeningChannel => [
    DeclaringConsumer,
    Connected,
    ClosingChannel,
    ReopeningChannel,
    ClosingConnection
]);
state_transition!(ReopeningChannel => OpeningChannel);

// Define states
struct Disconnected {}

impl Disconnected {
    async fn run(self) -> State {
        State::Connecting(Self::into_state(Connecting {}))
    }
}

struct Connecting {}

impl Connecting {
    async fn run(self, shared: &AmqpSharedData) -> State {
        tracing::info!(
            "connecting to amqp at {}://{}:{}/{} ...",
            shared.inner_opts.get_scheme().unwrap_or("amqp"),
            shared.inner_opts.get_host(),
            shared.inner_opts.get_port(),
            shared.inner_opts.get_virtual_host(),
        );

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        tracing::debug!("opening connection to amqp ...");
        let conn = match Connection::open(&shared.inner_opts).await {
            Ok(conn) => conn,
            Err(err) => {
                let err = format!("error connecting to amqp: {err}");
                tracing::error!(err);

                if shared.start_tx.is_some() {
                    return State::ClosedConnection(Self::into_state(ClosedConnection {
                        reason: CloseReason::StartError(Some(err.into())),
                    }));
                }

                return State::Reconnecting(Self::into_state(Reconnecting {}));
            }
        };

        tracing::debug!("registering connection callback handler ...");
        if let Err(err) = conn
            .register_callback(AmqpConnectionHandler::new(event_tx.clone()))
            .await
        {
            let err = format!("failed to register amqp connection callback: {err}");
            tracing::error!(err);

            if shared.start_tx.is_some() {
                return State::ClosingConnection(Self::into_state(ClosingConnection::new(
                    conn,
                    CloseReason::StartError(Some(err.into())),
                )));
            }

            return State::ClosingConnection(Self::into_state(ClosingConnection::new(
                conn,
                CloseReason::Other,
            )));
        };

        State::OpeningChannel(Self::into_state(OpeningChannel {
            conn,
            event_tx,
            event_rx,
        }))
    }
}

struct Reconnecting {}

impl Reconnecting {
    async fn run(self, shared: &AmqpSharedData) -> State {
        tracing::info!(
            "reconnecting in {}ms ...",
            shared.opts.reconnect_delay.as_millis(),
        );
        sleep(shared.opts.reconnect_delay).await;

        State::Connecting(Self::into_state(Connecting {}))
    }
}

struct OpeningChannel {
    conn: Connection,
    event_tx: mpsc::UnboundedSender<Event>,
    event_rx: mpsc::UnboundedReceiver<Event>,
}

impl OpeningChannel {
    fn connected(self, chan: Channel) -> State {
        State::Connected(Self::into_state(Connected {
            conn: self.conn,
            chan,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        }))
    }

    fn close_connection(self, reason: CloseReason) -> State {
        State::ClosingConnection(Self::into_state(ClosingConnection::new(self.conn, reason)))
    }

    fn reopen_channel(self) -> State {
        State::ReopeningChannel(Self::into_state(ReopeningChannel {
            conn: self.conn,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        }))
    }

    fn close_channel(self, chan: Channel, reason: CloseReason) -> State {
        State::ClosingChannel(Self::into_state(ClosingChannel::new(
            self.conn,
            chan,
            self.event_tx,
            self.event_rx,
            reason,
        )))
    }

    fn declare_consumer(self, chan: Channel) -> State {
        State::DeclaringConsumer(Self::into_state(DeclaringConsumer {
            conn: self.conn,
            chan,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        }))
    }

    async fn run(self, shared: &AmqpSharedData) -> State {
        tracing::debug!("opening channel ...");
        let chan = match self.conn.open_channel(None).await {
            Ok(chan) => chan,
            Err(err) => {
                let err = format!("couldn't create amqp channel: {err}");
                tracing::error!(err);

                if shared.start_tx.is_some() {
                    return self.close_connection(CloseReason::StartError(Some(err.into())));
                }

                return self.reopen_channel();
            }
        };

        tracing::debug!("registering channel callback handler ...");
        if let Err(err) = chan
            .register_callback(AmqpChannelHandler::new(self.event_tx.clone()))
            .await
        {
            let err = format!("failed to register amqp channel callback: {err}");
            tracing::error!(err);

            if shared.start_tx.is_some() {
                return self.close_channel(chan, CloseReason::StartError(Some(err.into())));
            }

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
            let err = format!(
                "error declaring queue '{}': {}",
                shared.opts.queue_name, err
            );
            tracing::error!(err);

            if shared.start_tx.is_some() {
                return self.close_channel(chan, CloseReason::StartError(Some(err.into())));
            }

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
    async fn run(self, shared: &AmqpSharedData) -> State {
        tracing::info!(
            "redeclaring channel in {}ms ...",
            shared.opts.reconnect_delay.as_millis(),
        );
        sleep(shared.opts.reconnect_delay).await;

        State::OpeningChannel(Self::into_state(OpeningChannel {
            conn: self.conn,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        }))
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
        State::Connected(Self::into_state(Connected {
            conn: self.conn,
            chan: self.chan,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        }))
    }

    fn close_channel(self, reason: CloseReason) -> State {
        State::ClosingChannel(Self::into_state(ClosingChannel::new(
            self.conn,
            self.chan,
            self.event_tx,
            self.event_rx,
            reason,
        )))
    }

    async fn run(self, shared: &AmqpSharedData) -> State {
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
            let err = format!("error declaring consumer: {err}");
            tracing::error!(err);

            if shared.start_tx.is_some() {
                return self.close_channel(CloseReason::StartError(Some(err.into())));
            }

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
        State::ClosingChannel(Self::into_state(ClosingChannel::new(
            self.conn,
            self.chan,
            self.event_tx,
            self.event_rx,
            reason,
        )))
    }

    async fn run(mut self, shared: &mut AmqpSharedData) -> State {
        if let Some(start_tx) = shared.start_tx.take() {
            if let Err(None) = start_tx.send(None) {
                tracing::warn!("start_tx::send couldn't send succesful start result");
            }
        }

        loop {
            select! {
                () = shared.shutdown.cancelled() => {
                    return self.close_channel(CloseReason::Shutdown);
                }
                event = self.event_rx.recv() => {
                    let Some(event) = event else {
                        tracing::error!("event_rx was closed, shutting down ...");
                        return self.close_channel(CloseReason::Fatal("event_rx was closed".into()));
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
                        return self.close_channel(CloseReason::Fatal("send_rx was closed".into()));
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
        State::ClosingConnection(Self::into_state(ClosingConnection::new(
            self.conn,
            self.reason,
        )))
    }

    fn reopen_channel(self) -> State {
        State::ReopeningChannel(Self::into_state(ReopeningChannel {
            conn: self.conn,
            event_tx: self.event_tx,
            event_rx: self.event_rx,
        }))
    }

    async fn run(mut self) -> State {
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
            | CloseReason::Fatal(_)
            | CloseReason::Other
            | CloseReason::Shutdown
            | CloseReason::StartError(_) => self.close_connection(),
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

    async fn run(mut self) -> State {
        assert!(
            self.conn.is_some(),
            "ClosingChannel::chan is None, shouldn't happen, did you not use ClosingChan::new?"
        );

        // NOTE: Can unwrap safely, we consume self, and ClosingChannel::new enforces Some(chan)
        if let Err(err) = self.conn.take().unwrap().close().await {
            tracing::warn!("error closing amqp conn: {err}");
        };

        State::ClosedConnection(Self::into_state(ClosedConnection {
            reason: self.reason,
        }))
    }
}

struct ClosedConnection {
    reason: CloseReason,
}

impl ClosedConnection {
    fn finished(self) -> State {
        State::Finished(self.reason)
    }

    fn finished_with_reason(self, reason: CloseReason) -> State {
        State::Finished(reason)
    }

    fn reconnect(self) -> State {
        State::Reconnecting(Self::into_state(Reconnecting {}))
    }

    async fn run(mut self, shared: &mut AmqpSharedData) -> State {
        match self.reason {
            CloseReason::ChannelClosed
            | CloseReason::PublishNoRoute
            | CloseReason::ConnectionClosed
            | CloseReason::Other => self.reconnect(),
            CloseReason::Fatal(_) | CloseReason::Shutdown => self.finished(),
            CloseReason::StartError(ref mut err) => {
                let Some(start_tx) = shared.start_tx.take() else {
                    let Some(err) = err.take() else {
                        return self.finished_with_reason(CloseReason::Fatal(
                            String::from("start_tx already consumed, and inner error is None, neither should happen")
                        ));
                    };

                    return self.finished_with_reason(CloseReason::Fatal(format!(
                        "start_tx already consumed, shouldn't happen, inner error: {err}"
                    )));
                };

                let Some(err) = err.take() else {
                    return self.finished_with_reason(CloseReason::Fatal(String::from(
                        "inner error is None, this shouldn't happen",
                    )));
                };

                if let Err(Some(err)) = start_tx.send(Some(err)) {
                    return self.finished_with_reason(CloseReason::Fatal(format!(
                        "couldn't send error back to user: {err}"
                    )));
                };

                self.finished_with_reason(CloseReason::Fatal("error while starting".into()))
            }
        }
    }
}
