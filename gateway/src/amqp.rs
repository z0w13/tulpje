use std::error::Error;

use amqprs::{
    channel::{BasicPublishArguments, Channel},
    connection::Connection,
    BasicProperties,
};

use tulpje_shared::amqp;

pub(crate) struct AmqprsProducer {
    #[expect(
        dead_code,
        reason = "we just don't want this to go out of scope, hence they're here"
    )]
    conn: Connection,
    chan: Channel,
}
impl AmqprsProducer {
    pub(crate) fn new(conn: Connection, chan: Channel) -> Self {
        Self { conn, chan }
    }

    pub(crate) async fn send(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        tracing::trace!(content_size = data.len(), "sending amqp message");

        self.chan
            .basic_publish(
                BasicProperties::default(),
                data.into(),
                BasicPublishArguments::new("", "discord"),
            )
            .await?;

        Ok(())
    }
}

pub(crate) async fn create(addr: &str) -> AmqprsProducer {
    let (conn, chan) = amqp::create(addr, "discord")
        .await
        .expect("couldn't create amqp client");

    AmqprsProducer::new(conn, chan)
}
