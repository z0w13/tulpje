use async_trait::async_trait;

use amqprs::{
    callbacks::ChannelCallback, channel::Channel, error::Error as AmqprsError, Ack,
    BasicProperties, Cancel, CloseChannel, Nack, Return,
};
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

pub(crate) struct AmqpChannelHandler {
    reconnect_token: CancellationToken,
    send_tx: UnboundedSender<Vec<u8>>,
}

impl AmqpChannelHandler {
    pub fn new(reconnect_token: CancellationToken, send_tx: UnboundedSender<Vec<u8>>) -> Self {
        Self {
            reconnect_token,
            send_tx,
        }
    }
}

#[async_trait]
impl ChannelCallback for AmqpChannelHandler {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<(), AmqprsError> {
        tracing::error!(
            "handle close request for channel {}, cause: {}",
            channel,
            close
        );
        self.reconnect_token.cancel();

        Ok(())
    }

    async fn cancel(&mut self, channel: &Channel, cancel: Cancel) -> Result<(), AmqprsError> {
        tracing::warn!(
            "handle cancel request for consumer {} on channel {}",
            cancel.consumer_tag(),
            channel
        );
        self.reconnect_token.cancel();

        Ok(())
    }

    async fn flow(&mut self, channel: &Channel, active: bool) -> Result<bool, AmqprsError> {
        tracing::info!(
            "handle flow request active={} for channel {}",
            active,
            channel
        );

        Ok(true)
    }

    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {
        tracing::info!(
            "handle publish ack delivery_tag={} on channel {}",
            ack.delivery_tag(),
            channel
        );
    }

    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {
        tracing::warn!(
            "handle publish nack delivery_tag={} on channel {}",
            nack.delivery_tag(),
            channel
        );
    }

    // handle publish return '312: NO_ROUTE', (exchange = , routing_key = discord) on channel 1 [open] of connection 'AMQPRS000@172.23.0.5:5672/ [open]', content size: 450
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        // 312 = NO_ROUTE, requeue
        if ret.reply_code() == 312 {
            tracing::warn!("couldn't route message, triggering reconnect and requeuing message");
            self.reconnect_token.cancel();
            if let Err(err) = self.send_tx.send(content) {
                tracing::error!("failed to requeue message: {err}");
            };
        } else {
            tracing::warn!(
                "handle publish return {} on channel {}, content size: {}",
                ret,
                channel,
                content.len()
            );
        }
    }
}
