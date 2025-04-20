use std::fmt::Display;

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
    Fatal,

    /// some other error occured, reconnecting
    // TODO: Add reason string
    Other,
}

impl Display for CloseReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Shutdown => "shutdown requested",
            Self::Fatal => "a fatal error occured",
            other => panic!("shut down with reason {other}, shouldn't happen"),
        })
    }
}
