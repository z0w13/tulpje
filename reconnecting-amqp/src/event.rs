use amqprs::{
    channel::Channel, connection::Connection, Ack, BasicProperties, Cancel, Close, CloseChannel,
    Deliver, Nack, Return,
};

pub enum Event {
    ConnectionClose(Connection, Close),
    ConnectionBlock(Connection, String),
    ConnectionUnblock(Connection),

    ChannelClose(Channel, CloseChannel),
    ChannelCancel(Channel, Cancel),
    ChannelFlow(Channel, bool),
    ChannelPublishAck(Channel, Ack),
    ChannelPublishNack(Channel, Nack),
    ChannelPublishReturn(Channel, Return, BasicProperties, Vec<u8>),

    MessageReceived(Channel, Deliver, BasicProperties, Vec<u8>),
}
