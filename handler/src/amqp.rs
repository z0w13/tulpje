#[cfg(feature = "amqp-amqprs")]
mod amqprs;
#[cfg(feature = "amqp-lapin")]
mod lapin;

#[cfg(feature = "amqp-amqprs")]
pub(crate) use amqprs::create;

#[cfg(feature = "amqp-lapin")]
pub(crate) use lapin::create;
