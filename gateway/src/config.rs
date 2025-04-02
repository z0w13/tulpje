use figment::{providers::Env, Figment};
use figment_file_provider_adapter::FileAdapter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub discord_token: String,
    pub discord_proxy: String,
    pub discord_gateway_queue: String,
    pub shard_id: u32,
    pub shard_count: u32,
    pub rabbitmq_address: String,
    pub redis_url: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Figment::new()
            .merge(FileAdapter::wrap(Env::raw()))
            .extract()?)
    }
}
