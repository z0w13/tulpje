use std::sync::Arc;

use redis::aio::ConnectionManager as RedisConnectionManager;

use tulpje_cache::Cache;
use tulpje_framework::{Registry, context};

#[derive(Clone)]
pub struct Services {
    pub handler_id: u32,

    pub cache: Arc<Cache>,
    // NOTE: Internally uses an Arc, "cheap" to clone
    pub redis: RedisConnectionManager,
    // NOTE: Internally uses an Arc, "cheap" to clone
    pub db: sqlx::PgPool,
    // NOTE: Cloning Registry would be very expensive and clones all the internal
    //       HashMaps, etc. so we should wrap it in an Arc
    pub registry: Arc<Registry<Services>>,
}

pub type ComponentInteractionContext = context::ComponentInteractionContext<Services>;
pub type CommandContext = context::CommandContext<Services>;
pub type EventContext = context::EventContext<Services>;
pub type TaskContext = context::TaskContext<Services>;
