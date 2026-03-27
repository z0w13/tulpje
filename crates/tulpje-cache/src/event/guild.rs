use twilight_model::gateway::payload::incoming::{
    GuildCreate, GuildDelete, GuildUpdate, UnavailableGuild,
};

use crate::UpdateCache;

impl UpdateCache for GuildCreate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        match self {
            Self::Available(guild) => cache.cache_guild(guild.clone()).await,
            Self::Unavailable(unavailable) => cache.unavailable_guild(unavailable.id).await,
        }
    }
}

impl UpdateCache for GuildDelete {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.delete_guild(self.id, false).await
    }
}

impl UpdateCache for GuildUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.update_guild(self).await
    }
}

impl UpdateCache for UnavailableGuild {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.unavailable_guild(self.id).await?;

        Ok(())
    }
}
