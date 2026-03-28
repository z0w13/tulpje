use serde::{Deserialize, Serialize};

use tulpje_framework::Metadata;

pub mod logging;
pub mod metrics;
pub mod shard_state;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEvent {
    pub meta: Metadata,
    pub payload: String,
}

impl DiscordEvent {
    pub fn new(shard: u32, payload: String) -> Self {
        Self {
            meta: Metadata::new(shard),
            payload,
        }
    }
}

#[macro_export]
macro_rules! version {
    () => {
        match option_env!("TULPJE_VERSION_EXTRA") {
            Some(extra) => format!("{} ({})", env!("CARGO_PKG_VERSION"), extra),
            _ => String::from(env!("CARGO_PKG_VERSION")),
        }
    };
}
