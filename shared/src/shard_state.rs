use std::time::{SystemTime, UNIX_EPOCH};

use bb8_redis::redis::{self, FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ShardState {
    pub shard_id: u32,
    pub guild_count: u64,

    pub up: bool,
    pub disconnect_count: u64,

    pub latency: u64,
    pub heartbeat_interval: u32,

    pub last_started: u64,
    pub last_heartbeat: u64,
    pub last_connection: u64,
}

impl ShardState {
    pub fn new(shard_id: u32) -> Self {
        Self {
            shard_id,
            last_started: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_secs(),

            ..Default::default()
        }
    }

    // heuristic way to determine whether the shard is up,
    // no heartbeats in heartbeat_interval * 1.2 = down
    pub fn is_up(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs();

        // a mess of converting but it'll do for now
        let heartbeat_interval_with_wiggle_room =
            ((f64::from(self.heartbeat_interval) / 1000.) * 1.2) as u64;
        self.up && now - self.last_heartbeat < heartbeat_interval_with_wiggle_room
    }
}

impl ToRedisArgs for ShardState {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(&serde_json::to_vec(self).expect("error serialising json"));
    }
}

impl FromRedisValue for ShardState {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn unix_now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
    }

    #[test]
    fn shard_state_is_up_test() {
        // if `up` is false we should be down
        let state = ShardState::new(0);
        assert!(!state.is_up());

        // if `up` is true but we have no heartbeat, should be down
        let mut state = ShardState::new(0);
        state.up = true;
        assert!(!state.is_up());

        // if `up` is true but we have no recent heartbeat, should be down
        let mut state = ShardState::new(0);
        state.up = true;
        state.last_heartbeat = unix_now() - 1_500;
        state.heartbeat_interval = 1_000;
        assert!(!state.is_up());

        // if `up` is true and we have recent heartbeat, should be up
        let mut state = ShardState::new(0);
        state.up = true;
        state.last_heartbeat = unix_now();
        state.heartbeat_interval = 1_000;
        assert!(state.is_up());
    }
}
