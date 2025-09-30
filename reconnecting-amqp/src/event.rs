use amqprs::{
    Ack, BasicProperties, Cancel, Close, CloseChannel, Deliver, Nack, Return, channel::Channel,
    connection::Connection,
};

pub enum Event {
    ConnectionClose(Connection, Close),
    ConnectionBlock(Connection, String),
    ConnectionUnblock(Connection),
    SecretUpdated(Connection),

    ChannelClose(Channel, CloseChannel),
    ChannelCancel(Channel, Cancel),
    ChannelFlow(Channel, bool),
    ChannelPublishAck(Channel, Ack),
    ChannelPublishNack(Channel, Nack),
    ChannelPublishReturn(Channel, Return, BasicProperties, Vec<u8>),

    MessageReceived(Channel, Deliver, BasicProperties, Vec<u8>),
}
