use std::collections::HashMap;

use bb8_redis::{redis::AsyncCommands as _, RedisConnectionManager};

use tulpje_framework::Error;
use tulpje_shared::{metrics::Metrics, shard_state::ShardState};

pub async fn get_all_shard_stats(
    redis: bb8::Pool<RedisConnectionManager>,
) -> Result<HashMap<u32, ShardState>, Error> {
    Ok(redis
        .get()
        .await?
        .hgetall::<&str, HashMap<String, ShardState>>("tulpje:shard_status")
        .await?
        .into_values()
        .map(|state| (state.shard_id, state))
        .collect())
}

pub async fn get_process_stats(
    redis: &bb8::Pool<RedisConnectionManager>,
    name: &str,
) -> Result<Option<Metrics>, Error> {
    Ok(redis
        .get()
        .await?
        .hget::<&str, &str, Option<Metrics>>("tulpje:metrics", name)
        .await?)
}

pub async fn get_all_process_stats(
    redis: bb8::Pool<RedisConnectionManager>,
) -> Result<HashMap<String, Metrics>, Error> {
    Ok(redis
        .get()
        .await?
        .hgetall::<&str, HashMap<String, Metrics>>("tulpje:metrics")
        .await?
        .into_values()
        .map(|metrics| (metrics.name.clone(), metrics))
        .collect())
}
