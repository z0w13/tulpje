use twilight_model::gateway::payload::incoming::GuildEmojisUpdate;

use crate::UpdateCache;

impl UpdateCache for GuildEmojisUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.cache_emojis(self.guild_id, self.emojis.clone()).await
    }
}
