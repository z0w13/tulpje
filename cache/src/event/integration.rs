use twilight_model::gateway::payload::incoming::{
    IntegrationCreate, IntegrationDelete, IntegrationUpdate,
};

use crate::{Cache, Error, UpdateCache};

impl UpdateCache for IntegrationCreate {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        if let Some(guild_id) = self.guild_id {
            cache.cache_integration(guild_id, &self.0).await?;
        }

        Ok(())
    }
}

impl UpdateCache for IntegrationDelete {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        cache.delete_integration(self.guild_id, self.id).await
    }
}

impl UpdateCache for IntegrationUpdate {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        if let Some(guild_id) = self.guild_id {
            cache.cache_integration(guild_id, &self.0).await?;
        }

        Ok(())
    }
}
