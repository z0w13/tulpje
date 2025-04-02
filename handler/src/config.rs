use figment::{providers::Env, Figment};
use figment_file_provider_adapter::FileAdapter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub discord_proxy: String,
    pub rabbitmq_address: String,
    pub redis_url: String,
    pub database_url: String,

    pub handler_id: u32,
    pub handler_count: u32,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Figment::new()
            .merge(FileAdapter::wrap(Env::raw()))
            .extract()?)
    }
}
