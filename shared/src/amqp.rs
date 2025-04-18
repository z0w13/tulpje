use std::error::Error;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{Channel, QueueDeclareArguments},
    connection::{Connection, OpenConnectionArguments},
};

async fn connect(addr: &str) -> Result<Connection, Box<dyn Error>> {
    let amqp_addr: OpenConnectionArguments = addr.try_into().expect("couldn't parse amqp uri");

    let amqp_conn = Connection::open(&amqp_addr)
        .await
        .map_err(|err| format!("error connecting to amqp: {err}"))?;
    amqp_conn
        .register_callback(DefaultConnectionCallback)
        .await
        .map_err(|err| format!("failed to register amqp connection callback: {err}"))?;

    Ok(amqp_conn)
}

async fn create_chan(conn: &Connection) -> Result<Channel, Box<dyn Error>> {
    let chan = conn
        .open_channel(None)
        .await
        .map_err(|err| format!("couldn't create amqp channel: {err}"))?;
    chan.register_callback(DefaultChannelCallback)
        .await
        .map_err(|err| format!("failed to register amqp channel callback: {err}"))?;

    Ok(chan)
}

pub async fn create(addr: &str, queue: &str) -> Result<(Connection, Channel), Box<dyn Error>> {
    let conn = connect(addr).await?;
    let chan = create_chan(&conn).await?;

    chan.queue_declare(QueueDeclareArguments::new(queue).durable(true).finish())
        .await
        .map_err(|err| format!("error declaring queue '{queue}': {err}"))?;

    Ok((conn, chan))
}
