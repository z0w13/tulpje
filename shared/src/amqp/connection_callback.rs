use amqprs::{
    callbacks::ConnectionCallback, connection::Connection, error::Error as AmqprsError, Close,
};
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

pub(crate) struct AmqpConnectionHandler {
    reconnect_token: CancellationToken,
}

impl AmqpConnectionHandler {
    pub fn new(reconnect_token: CancellationToken) -> Self {
        Self { reconnect_token }
    }
}

#[async_trait]
impl ConnectionCallback for AmqpConnectionHandler {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<(), AmqprsError> {
        tracing::error!(
            "handle close request for connection {}, cause: {}",
            connection,
            close
        );
        self.reconnect_token.cancel();

        Ok(())
    }

    async fn blocked(&mut self, connection: &Connection, reason: String) {
        tracing::info!(
            "handle blocked notification for connection {}, reason: {}",
            connection,
            reason
        );
    }

    async fn unblocked(&mut self, connection: &Connection) {
        tracing::info!(
            "handle unblocked notification for connection {}",
            connection
        );
    }
}
