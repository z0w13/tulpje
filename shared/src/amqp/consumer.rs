use amqprs::{channel::Channel, consumer::AsyncConsumer, BasicProperties, Deliver};
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::event::Event;

pub(crate) struct AmqpConsumer {
    event_tx: mpsc::UnboundedSender<Event>,
}

impl AmqpConsumer {
    pub(crate) fn new(event_tx: mpsc::UnboundedSender<Event>) -> Self {
        Self { event_tx }
    }
}

#[async_trait]
impl AsyncConsumer for AmqpConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) -> () {
        tracing::trace!(
            "consume delivery {} on channel {}, content size: {}",
            deliver,
            channel,
            content.len()
        );

        if let Err(err) = self.event_tx.send(Event::MessageReceived(
            channel.clone(),
            deliver,
            basic_properties,
            content,
        )) {
            tracing::error!("error sending Event::MessageReceived to queue: {err}");
        }
    }
}
