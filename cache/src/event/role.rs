use twilight_model::gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate};

use crate::UpdateCache;

impl UpdateCache for RoleCreate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.cache_role(self.guild_id, self.role.clone()).await?;
        Ok(())
    }
}

impl UpdateCache for RoleDelete {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.delete_role(self.guild_id, self.role_id).await?;
        Ok(())
    }
}

impl UpdateCache for RoleUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.cache_role(self.guild_id, self.role.clone()).await?;
        Ok(())
    }
}
