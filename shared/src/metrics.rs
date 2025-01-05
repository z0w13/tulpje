use std::{error::Error, time::Duration};

use bb8_redis::{
    redis::{self, AsyncCommands as _, FromRedisValue, ToRedisArgs},
    RedisConnectionManager,
};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_process::Collector as ProcessCollector;
use serde::{Deserialize, Serialize};

pub fn install(
    builder: PrometheusBuilder,
    redis: bb8::Pool<RedisConnectionManager>,
    process_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // install recorder and exporter
    builder
        .add_global_label("process", &process_name)
        .install()?;

    // define and start process metrics
    let proc_collector = ProcessCollector::default();
    proc_collector.describe();

    // create and run metrics manager
    MetricsManager::new(process_name, redis, proc_collector).run();

    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Metrics {
    pub name: String,

    pub cpu_usage: f32,
    pub memory_usage: u64,
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

#[derive(Clone)]
struct MetricsManager {
    name: String,

    interval_ms: u64,
    prev_cpu_ms: u64,

    redis: bb8::Pool<RedisConnectionManager>,
    collector: ProcessCollector,
}

impl MetricsManager {
    fn new(
        name: String,
        redis: bb8::Pool<RedisConnectionManager>,
        collector: ProcessCollector,
    ) -> Self {
        Self {
            name,

            interval_ms: 10_000,
            prev_cpu_ms: 0,

            redis,
            collector,
        }
    }

    fn run(&self) {
        let mut manager = self.clone();

        tokio::spawn(async move {
            loop {
                if let Err(err) = manager.update().await {
                    tracing::error!("error updating shard metrics: {}", err);
                }
                tokio::time::sleep(Duration::from_millis(manager.interval_ms)).await;
            }
        });
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "we never encounter numbers large enough to cause precision loss"
    )]
    async fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.collector.collect();

        let metrics = metrics_process::collector::collect();

        let memory_usage = metrics.resident_memory_bytes.unwrap_or(0);
        let curr_cpu_ms = (metrics.cpu_seconds_total.unwrap_or(0.) * 1000.) as u64;

        // NOTE: these would only overflow if either has a number over 285_000 years
        //       and if we're running or waiting for that long, there's other concerns
        let interval_ms_float = self.interval_ms as f32;
        let cpu_usage = (curr_cpu_ms - self.prev_cpu_ms) as f32 / interval_ms_float;
        self.prev_cpu_ms = curr_cpu_ms;

        // TODO: Implement IDs for the instances
        self.redis
            .get()
            .await?
            .hset::<&str, &str, Metrics, ()>(
                "tulpje:metrics",
                &self.name,
                Metrics {
                    name: self.name.clone(),

                    cpu_usage,
                    memory_usage,
                },
            )
            .await?;

        Ok(())
    }
}
