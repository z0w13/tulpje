use async_trait::async_trait;

use super::event::Event;
use amqprs::{
    callbacks::ChannelCallback, channel::Channel, error::Error as AmqprsError, Ack,
    BasicProperties, Cancel, CloseChannel, Nack, Return,
};
use tokio::sync::mpsc;

pub(crate) struct AmqpChannelHandler {
    event_tx: mpsc::UnboundedSender<Event>,
}

impl AmqpChannelHandler {
    pub(crate) fn new(event_tx: mpsc::UnboundedSender<Event>) -> Self {
        Self { event_tx }
    }
}

#[async_trait]
impl ChannelCallback for AmqpChannelHandler {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<(), AmqprsError> {
        tracing::debug!("close request for channel {}, cause: {}", channel, close);

        if let Err(err) = self
            .event_tx
            .send(Event::ChannelClose(channel.clone(), close))
        {
            tracing::error!("error sending Event::ChannelClose to queue: {err}");
        }

        Ok(())
    }

    async fn cancel(&mut self, channel: &Channel, cancel: Cancel) -> Result<(), AmqprsError> {
        tracing::debug!(
            "cancel request for consumer {} on channel {}",
            cancel.consumer_tag(),
            channel
        );

        if let Err(err) = self
            .event_tx
            .send(Event::ChannelCancel(channel.clone(), cancel))
        {
            tracing::error!("error sending Event::ChannelCancel to queue: {err}");
        }

        Ok(())
    }

    async fn flow(&mut self, channel: &Channel, active: bool) -> Result<bool, AmqprsError> {
        tracing::debug!("flow request active={} for channel {}", active, channel);

        if let Err(err) = self
            .event_tx
            .send(Event::ChannelFlow(channel.clone(), active))
        {
            tracing::error!("error sending Event::ChannelFlow to queue: {err}");
        }

        Ok(true)
    }

    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {
        tracing::debug!(
            "publish ack delivery_tag={} on channel {}",
            ack.delivery_tag(),
            channel
        );

        if let Err(err) = self
            .event_tx
            .send(Event::ChannelPublishAck(channel.clone(), ack))
        {
            tracing::error!("error sending Event::ChannelPublishAck to queue: {err}");
        }
    }

    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {
        tracing::debug!(
            "publish nack delivery_tag={} on channel {}",
            nack.delivery_tag(),
            channel
        );

        if let Err(err) = self
            .event_tx
            .send(Event::ChannelPublishNack(channel.clone(), nack))
        {
            tracing::error!("error sending Event::ChannelPublishNack to queue: {err}");
        }
    }

    // handle publish return '312: NO_ROUTE', (exchange = , routing_key = discord) on channel 1 [open] of connection 'AMQPRS000@172.23.0.5:5672/ [open]', content size: 450
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        // // 312 = NO_ROUTE, requeue
        // if ret.reply_code() == 312 {
        //     tracing::warn!("couldn't route message, triggering reconnect and requeuing message");
        //     self.reconnect_token.cancel();
        //     if let Err(err) = self.send_tx.send(content) {
        //         tracing::error!("failed to requeue message: {err}");
        //     };
        // }
        //
        tracing::debug!(
            "publish return {} on channel {}, content size: {}",
            ret,
            channel,
            content.len()
        );

        if let Err(err) = self.event_tx.send(Event::ChannelPublishReturn(
            channel.clone(),
            ret,
            basic_properties,
            content,
        )) {
            tracing::error!("error sending Event::ChannelPublishReturn to queue: {err}");
        }
    }
}
