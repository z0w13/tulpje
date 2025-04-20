mod channel_callback;
mod connection_callback;
mod consumer;
pub mod event;

use std::{error::Error, time::Duration};

use amqprs::{
    channel::{BasicConsumeArguments, BasicPublishArguments, Channel, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use consumer::AmqpConsumer;
use event::Event;
use tokio::{
    select,
    sync::mpsc::{self, UnboundedSender},
    time::sleep,
};

use channel_callback::AmqpChannelHandler;
use connection_callback::AmqpConnectionHandler;

pub struct ConnectionArguments {
    reconnect_time: Duration,
    queue_name: String,
}

enum ConnectionState {
    Disconnected,
    Connected(Connection, Channel, mpsc::UnboundedReceiver<Event>),
}

pub struct AmqpConnection {
    opts: ConnectionArguments,
    inner_opts: OpenConnectionArguments,

    recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,

    send_tx: mpsc::UnboundedSender<Vec<u8>>,
    send_rx: mpsc::UnboundedReceiver<Vec<u8>>,
}

impl AmqpConnection {
    pub fn new(
        inner_opts: OpenConnectionArguments,
        opts: ConnectionArguments,
        recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
    ) -> Self {
        let (send_tx, send_rx) = mpsc::unbounded_channel::<Vec<u8>>();

        Self {
            opts,
            inner_opts,

            recv_tx,

            send_tx,
            send_rx,
        }
    }

    /// create a new AmqpConnection from a string address
    pub fn from_str(
        addr: &str,
        opts: ConnectionArguments,
        recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self::new(
            addr.try_into()
                .map_err(|err| format!("couldn't parse amqp uri: {err}"))?,
            opts,
            recv_tx,
        ))
    }

    /// connect to amqp
    async fn connect(&mut self) -> Result<ConnectionState, Box<dyn Error + Send + Sync>> {
        tracing::info!(
            "connecting to amqp at {}://{}:{}/{} ...",
            self.inner_opts.get_scheme().unwrap_or("amqp"),
            self.inner_opts.get_host(),
            self.inner_opts.get_port(),
            self.inner_opts.get_virtual_host(),
        );

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        tracing::debug!("opening connection to amqp ...");
        let conn = Connection::open(&self.inner_opts)
            .await
            .map_err(|err| format!("error connecting to amqp: {err}"))?;
        tracing::debug!("registering connection callback handler ...");
        conn.register_callback(AmqpConnectionHandler::new(event_tx.clone()))
            .await
            .map_err(|err| format!("failed to register amqp connection callback: {err}"))?;

        self.declare_channel(conn, event_tx, event_rx).await
    }

    async fn declare_channel(
        &mut self,
        conn: Connection,
        event_tx: mpsc::UnboundedSender<Event>,
        event_rx: mpsc::UnboundedReceiver<Event>,
    ) -> Result<ConnectionState, Box<dyn Error + Send + Sync>> {
        tracing::debug!("opening channel ...");
        let chan = conn
            .open_channel(None)
            .await
            .map_err(|err| format!("couldn't create amqp channel: {err}"))?;
        tracing::debug!("registering channel callback handler ...");
        chan.register_callback(AmqpChannelHandler::new(event_tx.clone()))
            .await
            .map_err(|err| format!("failed to register amqp channel callback: {err}"))?;

        tracing::debug!("declaring queue {} ...", self.opts.queue_name);
        chan.queue_declare(
            QueueDeclareArguments::new(&self.opts.queue_name)
                .durable(true)
                .finish(),
        )
        .await
        .map_err(|err| format!("error declaring queue '{}': {}", self.opts.queue_name, err))?;

        if self.recv_tx.is_some() {
            self.declare_consumer(conn, chan, event_tx, event_rx).await
        } else {
            Ok(ConnectionState::Connected(conn, chan, event_rx))
        }
    }

    async fn declare_consumer(
        &mut self,
        conn: Connection,
        chan: Channel,
        event_tx: mpsc::UnboundedSender<Event>,
        event_rx: mpsc::UnboundedReceiver<Event>,
    ) -> Result<ConnectionState, Box<dyn Error + Send + Sync>> {
        tracing::debug!("declaring amqp consumer ...");
        chan.basic_consume(
            AmqpConsumer::new(event_tx.clone()),
            BasicConsumeArguments::new(&self.opts.queue_name, "")
                .manual_ack(false)
                .finish(),
        )
        .await
        .map_err(|err| format!("error declaring consumer: {err}"))?;

        Ok(ConnectionState::Connected(conn, chan, event_rx))
    }

    /// inner processing loop for the amqp connection
    async fn inner_loop(
        &mut self,
        conn: Connection,
        chan: Channel,
        mut event_rx: mpsc::UnboundedReceiver<Event>,
    ) -> ConnectionState {
        loop {
            select! {
                event = event_rx.recv() => {
                    let Some(event) = event else {
                        tracing::warn!("event_rx was closed, reconnecting ...");
                        break
                    };
                    match event {
                        Event::ConnectionClose(_, _) | Event::ChannelClose(_, _) => {
                            if let Err(err) = chan.close().await {
                                tracing::warn!("error closing amqp conn: {err}");
                            }

                            if let Err(err) = conn.close().await {
                                tracing::warn!("error closing amqp conn: {err}");
                            }

                            break;
                        }
                        Event::ChannelPublishReturn(_chan, ret, _basic_properties, data) => {
                            if ret.reply_code() == 312 {
                                tracing::warn!("couldn't route message, triggering reconnect and requeuing message");
                                if let Err(err) = self.send_tx.send(data) {
                                    tracing::error!("failed to requeue message: {err}");
                                };
                                break
                            }
                        }
                        Event::MessageReceived(_chan, _deliver, _basic_properties, data) => {
                            if let Some(recv_tx) = &self.recv_tx {
                                if let Err(err) = recv_tx.send(data) {
                                    tracing::warn!("error sending received message to library user: {err}");
                                }
                            }
                        }
                        _ => {}
                    }
                }
                message = self.send_rx.recv() => {
                    let Some(message) = message else {
                        tracing::error!("send_rx was closed, this shouldn't really happen ...");
                        break
                    };

                    if let Err(err) = chan.basic_publish(
                        BasicProperties::default(),
                        message,
                        // We set the mandatory flag so we can handle when the
                        // message can't be delivered to any queues, and then assume
                        // our queue was deleted and force a reconnect in
                        // ChannelCallback::publish_return
                        BasicPublishArguments::new("", &self.opts.queue_name).mandatory(true).finish(),
                    )
                    .await {
                        tracing::error!("error sending event to amqp: {}", err);
                    }
                }
            }
        }

        ConnectionState::Disconnected
    }

    /// return a channel for applications to send messages
    pub fn send_chan(&self) -> UnboundedSender<Vec<u8>> {
        self.send_tx.clone()
    }

    /// start the amqp connection
    pub async fn run(&mut self) {
        let mut state = ConnectionState::Disconnected;
        loop {
            state = match state {
                ConnectionState::Disconnected => {
                    // try to connect
                    match self.connect().await {
                        Ok(state) => state,
                        Err(err) => {
                            // if we fail wait for opts.reconnect_time and then retry
                            tracing::error!("couldn't connect to amqp: {err}");
                            tracing::info!(
                                "reconnecting in {}ms ...",
                                self.opts.reconnect_time.as_millis()
                            );
                            sleep(self.opts.reconnect_time).await;

                            ConnectionState::Disconnected
                        }
                    }
                }
                ConnectionState::Connected(conn, chan, event_rx) => {
                    self.inner_loop(conn, chan, event_rx).await
                }
            }
        }
    }
}

pub async fn create(
    addr: &str,
    queue_name: &str,
    recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
) -> Result<AmqpConnection, Box<dyn Error + Send + Sync>> {
    AmqpConnection::from_str(
        addr,
        ConnectionArguments {
            reconnect_time: Duration::new(2, 0),
            queue_name: String::from(queue_name),
        },
        recv_tx,
    )
}
