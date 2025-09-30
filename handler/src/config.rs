use figment::{Figment, providers::Env};
use figment_file_provider_adapter::FileAdapter;
use serde::{Deserialize, Serialize};

use tulpje_shared::metrics::MetricsListenAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub discord_proxy: String,
    pub rabbitmq_address: String,
    pub redis_url: String,
    pub database_url: String,

    pub handler_id: u32,
    pub handler_count: u32,

    #[serde(default = "MetricsListenAddr::default")]
    pub metrics_listen_addr: MetricsListenAddr,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Figment::new()
            .merge(FileAdapter::wrap(Env::raw()))
            .extract()?)
    }
}
