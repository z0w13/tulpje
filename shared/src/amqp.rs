mod channel_callback;
mod close_reason;
mod connection;
mod connection_callback;
mod consumer;
pub mod event;

use std::time::Duration;

use amqprs::connection::OpenConnectionArguments;
use connection::AmqpConnection;
use consumer::AmqpConsumer;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

use channel_callback::AmqpChannelHandler;
use connection_callback::AmqpConnectionHandler;
use tokio_util::sync::CancellationToken;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct ConnectionArguments {
    reconnect_delay: Duration,
    queue_name: String,
}

impl ConnectionArguments {
    pub fn new(queue_name: impl Into<String>) -> Self {
        Self {
            reconnect_delay: Duration::new(2, 0),
            queue_name: queue_name.into(),
        }
    }

    pub fn reconnect_delay(mut self, reconnect_delay: Duration) -> Self {
        self.reconnect_delay = reconnect_delay;
        self
    }
}

pub struct AmqpHandle {
    send_tx: mpsc::UnboundedSender<Vec<u8>>,
    start: CancellationToken,
    start_rx: Option<oneshot::Receiver<Option<Error>>>,
    shutdown: CancellationToken,
    handle: Option<JoinHandle<()>>,
}

impl AmqpHandle {
    pub fn new(
        inner_opts: OpenConnectionArguments,
        opts: ConnectionArguments,
        recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
    ) -> Self {
        let (send_tx, send_rx) = mpsc::unbounded_channel();
        let (start_tx, start_rx) = oneshot::channel();
        let start = CancellationToken::new();
        let shutdown = CancellationToken::new();

        let amqp = AmqpConnection::new(
            opts,
            inner_opts,
            recv_tx,
            send_tx.clone(),
            send_rx,
            start.clone(),
            start_tx,
            shutdown.clone(),
        );

        let handle = Some(tokio::spawn(async move {
            amqp.run().await;
        }));

        Self {
            send_tx,
            start,
            start_rx: Some(start_rx),
            shutdown,
            handle,
        }
    }

    /// create a new AmqpConnection from a string address
    pub fn try_from_str(
        addr: &str,
        opts: ConnectionArguments,
        recv_tx: Option<mpsc::UnboundedSender<Vec<u8>>>,
    ) -> Result<Self, Error> {
        Ok(Self::new(
            addr.try_into()
                .map_err(|err| format!("couldn't parse amqp uri: {err}"))?,
            opts,
            recv_tx,
        ))
    }

    pub fn start(&mut self) {
        self.start.cancel();
    }

    pub async fn wait_start(&mut self) -> Result<(), Error> {
        self.start.cancel();

        let Some(start_rx) = self.start_rx.take() else {
            return Err(
                "`start_rx` is none, have you called `AmqpHandle::wait_start` before?".into(),
            );
        };

        match start_rx.await {
            Ok(None) => Ok(()),
            Ok(Some(err)) => Err(err),
            Err(err) => Err(err.into()),
        }
    }

    pub fn shutdown(&mut self) {
        self.shutdown.cancel();
    }

    pub async fn join(&mut self) -> Result<(), Error> {
        Ok(self
            .handle
            .take()
            .ok_or("AmqpConnection already shutdown")?
            .await?)
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<Vec<u8>> {
        self.send_tx.clone()
    }
}
