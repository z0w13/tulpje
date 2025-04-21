use std::fmt::Display;

use super::Error;

pub(crate) enum CloseReason {
    /// we're shutting down, don't reconnect
    Shutdown,

    /// channel was closed, redeclaring
    ChannelClosed,

    /// connection was closed, reconnecting
    ConnectionClosed,

    /// a message we tried to send couldn't be sent to consumers, this usually
    /// means the queue got deleted, so redeclare the channel
    // TODO: Only redeclare the queue and don't recreate the entire channel
    PublishNoRoute,

    /// a fatal error occured and there's no way we can recover, don't reconnect
    // TODO: Add reason string
    Fatal(String),

    /// some other error occured, reconnecting
    // TODO: Add reason string
    Other,

    /// special close reason so we can pass the error to whoever is waiting on it
    StartError(Option<Error>),
}

impl Display for CloseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Self::Shutdown => String::from("shutdown requested"),
            Self::Fatal(reason) => format!("a fatal error occured: {reason}"),
            other => panic!("shut down with reason {other}, shouldn't happen"),
        })
    }
}
