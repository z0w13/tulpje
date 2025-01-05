use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub uuid: uuid::Uuid, // used for tracing
    pub shard: u32,
}

impl Metadata {
    pub fn new(shard: u32) -> Self {
        Self {
            uuid: uuid::Uuid::now_v7(),
            shard,
        }
    }
}
