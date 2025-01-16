use twilight_model::gateway::payload::incoming::GuildStickersUpdate;

use crate::UpdateCache;

impl UpdateCache for GuildStickersUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache
            .cache_stickers(self.guild_id, self.stickers.clone())
            .await?;
        Ok(())
    }
}
