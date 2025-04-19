use amqprs::{channel::Channel, consumer::AsyncConsumer, BasicProperties, Deliver};
use async_trait::async_trait;
use tokio::sync::mpsc;

pub(crate) struct AmqpConsumer {
    queue: mpsc::UnboundedSender<Vec<u8>>,
}

impl AmqpConsumer {
    pub(crate) fn new(queue: mpsc::UnboundedSender<Vec<u8>>) -> Self {
        Self { queue }
    }
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
