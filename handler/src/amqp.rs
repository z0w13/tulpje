use async_trait::async_trait;
use tokio::sync::mpsc;

use amqprs::{
    channel::{BasicConsumeArguments, Channel},
    connection::Connection,
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};

use tulpje_shared::amqp;

pub(crate) struct AmqprsConsumer {
    queue: mpsc::UnboundedReceiver<Vec<u8>>,
    #[expect(
        dead_code,
        reason = "we just don't want them to go out of scope, hence they're here"
    )]
    conn: Connection,
    #[expect(
        dead_code,
        reason = "we just don't want them to go out of scope, hence they're here"
    )]
    chan: Channel,
}
impl AmqprsConsumer {
    pub(crate) async fn recv(&mut self) -> Option<Vec<u8>> {
        self.queue.recv().await
    }
}

pub(crate) async fn create(addr: &str) -> AmqprsConsumer {
    let (conn, chan) = amqp::create(addr, "discord")
        .await
        .expect("couldn't create amqp client");

    let (message_queue_send, message_queue_recv) = mpsc::unbounded_channel::<Vec<u8>>();
    chan.basic_consume(
        AmqpConsumer {
            queue: message_queue_send,
        },
        BasicConsumeArguments::new("discord", "")
            .manual_ack(false)
            .finish(),
    )
    .await
    .expect("error declaring amqp consumer");

    AmqprsConsumer {
        conn,
        chan,
        queue: message_queue_recv,
    }
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
