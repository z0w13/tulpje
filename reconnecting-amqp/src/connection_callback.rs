use amqprs::{
    Close, callbacks::ConnectionCallback, connection::Connection, error::Error as AmqprsError,
};
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::event::Event;

pub(crate) struct AmqpConnectionHandler {
    event_tx: mpsc::UnboundedSender<Event>,
}

impl AmqpConnectionHandler {
    pub(crate) fn new(event_tx: mpsc::UnboundedSender<Event>) -> Self {
        Self { event_tx }
    }
}

#[async_trait]
impl ConnectionCallback for AmqpConnectionHandler {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<(), AmqprsError> {
        tracing::debug!(
            "close request for connection {}, cause: {}",
            connection,
            close
        );

        if let Err(err) = self
            .event_tx
            .send(Event::ConnectionClose(connection.clone(), close))
        {
            tracing::error!("error sending Event::ConnectionClose to queue: {err}");
        }

        Ok(())
    }

    async fn blocked(&mut self, connection: &Connection, reason: String) {
        tracing::debug!(
            "blocked notification for connection {}, reason: {}",
            connection,
            reason
        );

        if let Err(err) = self
            .event_tx
            .send(Event::ConnectionBlock(connection.clone(), reason))
        {
            tracing::error!("error sending Event::ConnectionBlock to queue: {err}");
        }
    }

    async fn unblocked(&mut self, connection: &Connection) {
        tracing::debug!("unblocked notification for connection {}", connection);

        if let Err(err) = self
            .event_tx
            .send(Event::ConnectionUnblock(connection.clone()))
        {
            tracing::error!("error sending Event::ConnectionUnblock to queue: {err}");
        }
    }

    async fn secret_updated(&mut self, connection: &Connection) {
        tracing::debug!("secret updated for connection {}", connection);

        if let Err(err) = self.event_tx.send(Event::SecretUpdated(connection.clone())) {
            tracing::error!("error sending Event::SecretUpdated to queue: {err}");
        }
    }
}
