use std::collections::HashMap;

use redis::{AsyncCommands as _, aio::ConnectionManager as RedisConnectionManager};

use tulpje_framework::Error;
use tulpje_shared::{metrics::Metrics, shard_state::ShardState};

pub async fn get_all_shard_stats(
    redis: RedisConnectionManager,
) -> Result<HashMap<u32, ShardState>, Error> {
    Ok(redis
        .clone()
        .hgetall::<&str, HashMap<String, ShardState>>("tulpje:shard_status")
        .await?
        .into_values()
        .map(|state| (state.shard_id, state))
        .collect())
}

pub async fn get_process_stats(
    redis: &RedisConnectionManager,
    name: &str,
) -> Result<Option<Metrics>, Error> {
    Ok(redis
        .clone()
        .hget::<&str, &str, Option<Metrics>>("tulpje:metrics", name)
        .await?)
}

pub async fn get_all_process_stats(
    redis: RedisConnectionManager,
) -> Result<HashMap<String, Metrics>, Error> {
    Ok(redis
        .clone()
        .hgetall::<&str, HashMap<String, Metrics>>("tulpje:metrics")
        .await?
        .into_values()
        .map(|metrics| (metrics.name.clone(), metrics))
        .collect())
}
