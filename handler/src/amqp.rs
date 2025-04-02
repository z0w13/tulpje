use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{BasicConsumeArguments, Channel, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};

use async_trait::async_trait;
use tokio::sync::mpsc;

pub(crate) struct AmqprsConsumer {
    queue: mpsc::UnboundedReceiver<Vec<u8>>,
    conn: Option<Connection>,
    chan: Option<Channel>,
}
impl AmqprsConsumer {
    pub(crate) fn new(
        queue: mpsc::UnboundedReceiver<Vec<u8>>,
        conn: Connection,
        chan: Channel,
    ) -> Self {
        Self {
            queue,
            conn: Some(conn),
            chan: Some(chan),
        }
    }
    pub(crate) async fn recv(&mut self) -> Option<Vec<u8>> {
        self.queue.recv().await
    }

    pub(crate) async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.chan
            .take()
            .ok_or("chan is None, did you already call AmqprsConsumer::shutdown?")?
            .close()
            .await?;

        self.conn
            .take()
            .ok_or("conn is None, did you already call AmqprsConsumer::shutdown?")?
            .close()
            .await?;

        self.queue.close();

        Ok(())
    }
}

pub(crate) async fn create(addr: &str) -> AmqprsConsumer {
    let amqp_addr: OpenConnectionArguments = addr.try_into().expect("couldn't parse amqp uri");

    let amqp_conn = Connection::open(&amqp_addr)
        .await
        .expect("error connecting to amqp");
    amqp_conn
        .register_callback(DefaultConnectionCallback)
        .await
        .expect("failed to register amqp connection callback");

    let amqp_chan = amqp_conn
        .open_channel(None)
        .await
        .expect("couldn't create amqp channel");
    amqp_chan
        .register_callback(DefaultChannelCallback)
        .await
        .expect("failed to register amqp channel callback");
    amqp_chan
        .queue_declare(QueueDeclareArguments::new("discord").durable(true).finish())
        .await
        .expect("error declaring 'discord' amqp queue");

    let (message_queue_send, message_queue_recv) = mpsc::unbounded_channel::<Vec<u8>>();
    amqp_chan
        .basic_consume(
            AmqpConsumer {
                queue: message_queue_send,
            },
            BasicConsumeArguments::new("discord", "")
                .manual_ack(false)
                .finish(),
        )
        .await
        .expect("error declaring amqp consumer");

    AmqprsConsumer::new(message_queue_recv, amqp_conn, amqp_chan)
}

struct AmqpConsumer {
    queue: mpsc::UnboundedSender<Vec<u8>>,
}

#[async_trait]
impl AsyncConsumer for AmqpConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) -> () {
        tracing::trace!(
            "consume delivery {} on channel {}, content size: {}",
            deliver,
            channel,
            content.len()
        );

        if let Err(err) = self.queue.send(content) {
            tracing::error!("error putting message on queue: {}", err);
        }
    }
}
