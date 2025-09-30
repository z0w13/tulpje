use std::{
    error::Error,
    fmt::Display,
    net::SocketAddr,
    str::FromStr as _,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_process::Collector as ProcessCollector;
use redis::{
    self, AsyncCommands as _, FromRedisValue, ToRedisArgs,
    aio::ConnectionManager as RedisConnectionManager,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsListenAddr(SocketAddr);

impl Default for MetricsListenAddr {
    fn default() -> Self {
        Self(SocketAddr::from_str("0.0.0.0:9000").expect("this socket addr should always be valid"))
    }
}

impl Display for MetricsListenAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<MetricsListenAddr> for SocketAddr {
    fn from(value: MetricsListenAddr) -> Self {
        value.0
    }
}

pub fn install(
    builder: PrometheusBuilder,
    listen_addr: MetricsListenAddr,
    redis: RedisConnectionManager,
    process_name: String,
    version: String,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("creating prometheus endpoint on http://{}", listen_addr);

    // install recorder and exporter
    builder
        .with_http_listener(listen_addr)
        .add_global_label("process", &process_name)
        .install()?;

    // define and start process metrics
    let proc_collector = ProcessCollector::default();
    proc_collector.describe();

    // create and run metrics manager
    tokio::spawn(async {
        MetricsManager::new(process_name, version, redis, proc_collector)
            .run()
            .await;
    });

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Metrics {
    pub name: String,
    pub version: String,

    pub cpu_usage: f32,
    pub memory_usage: u64,

    pub last_started: u64,
    pub last_updated: u64,
}

impl Metrics {
    fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            last_started: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs(),

            ..Default::default()
        }
    }
}

impl ToRedisArgs for Metrics {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(&serde_json::to_vec(self).expect("error serialising json"));
    }
}

impl FromRedisValue for Metrics {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::BulkString(ref bytes) => match serde_json::from_slice(bytes) {
                Ok(rv) => Ok(rv),
                Err(err) => Err(redis::RedisError::from((
                    redis::ErrorKind::TypeError,
                    "error deserializing json",
                    format!("{err}"),
                ))),
            },
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "invalid response type for json",
                format!("{:?}", v),
            ))),
        }
    }
}

struct MetricsManager {
    metrics: Metrics,

    interval_ms: u64,
    prev_cpu_ms: u64,

    redis: RedisConnectionManager,
    collector: ProcessCollector,
}

impl MetricsManager {
    fn new(
        name: String,
        version: String,
        redis: RedisConnectionManager,
        collector: ProcessCollector,
    ) -> Self {
        Self {
            metrics: Metrics::new(name, version),

            interval_ms: 10_000,
            prev_cpu_ms: 0,

            redis,
            collector,
        }
    }

    async fn run(&mut self) {
        loop {
            if let Err(err) = self.update().await {
                tracing::error!("error updating shard metrics: {}", err);
            }
            tokio::time::sleep(Duration::from_millis(self.interval_ms)).await;
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "we never encounter numbers large enough to cause precision loss"
    )]
    async fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.collector.collect();

        // collect data from metrics_process collector for local use
        // ugly double work, but it works
        let metrics = metrics_process::collector::collect();

        let curr_cpu_ms = (metrics.cpu_seconds_total.unwrap_or(0.) * 1000.) as u64;
        // NOTE: these would only overflow if either has a number over 285_000 years
        //       and if we're running or waiting for that long, there's other concerns
        let interval_ms_float = self.interval_ms as f32;
        self.prev_cpu_ms = curr_cpu_ms;

        // Update metrics
        self.metrics.cpu_usage = (curr_cpu_ms - self.prev_cpu_ms) as f32 / interval_ms_float;
        self.metrics.memory_usage = metrics.resident_memory_bytes.unwrap_or(0);
        self.metrics.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs();

        // TODO: Implement IDs for the instances
        self.redis
            .hset::<&str, &str, &Metrics, ()>("tulpje:metrics", &self.metrics.name, &self.metrics)
            .await?;

        Ok(())
    }
}
