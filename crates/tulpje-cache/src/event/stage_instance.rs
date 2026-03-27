use twilight_model::gateway::payload::incoming::{
    StageInstanceCreate, StageInstanceDelete, StageInstanceUpdate,
};

use crate::UpdateCache;

impl UpdateCache for StageInstanceCreate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache
            .cache_stage_instance(self.guild_id, self.0.clone())
            .await?;
        Ok(())
    }
}
impl UpdateCache for StageInstanceDelete {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.delete_stage_instance(self.guild_id, self.id).await?;
        Ok(())
    }
}

impl UpdateCache for StageInstanceUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache
            .cache_stage_instance(self.guild_id, self.0.clone())
            .await?;
        Ok(())
    }
}
