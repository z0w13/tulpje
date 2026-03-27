use twilight_model::gateway::payload::incoming::PresenceUpdate;

use crate::{Cache, Error, UpdateCache};

impl UpdateCache for PresenceUpdate {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        cache.cache_presence(self.guild_id, self.0.clone()).await
    }
}
