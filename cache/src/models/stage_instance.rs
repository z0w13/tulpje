use twilight_model::{
    channel::StageInstance,
    id::{
        Id,
        marker::{GuildMarker, StageMarker},
    },
};

use crate::{Cache, Error, GuildResource};

pub use twilight_model::channel::StageInstance as CachedStageInstance;

impl Cache {
    pub(crate) async fn cache_stage_instances(
        &self,
        guild_id: Id<GuildMarker>,
        stage_instances: impl IntoIterator<Item = StageInstance>,
    ) -> Result<(), Error> {
        for stage_instance in stage_instances {
            self.cache_stage_instance(guild_id, stage_instance).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_stage_instance(
        &self,
        guild_id: Id<GuildMarker>,
        stage_instance: StageInstance,
    ) -> Result<(), Error> {
        self.guild_stage_instances
            .insert(&guild_id, &stage_instance.id)
            .await?;

        self.stage_instances
            .insert(
                &stage_instance.id.clone(),
                &GuildResource {
                    guild_id,
                    value: stage_instance,
                },
            )
            .await?;

        Ok(())
    }

    pub(crate) async fn delete_stage_instance(
        &self,
        guild_id: Id<GuildMarker>,
        stage_id: Id<StageMarker>,
    ) -> Result<(), Error> {
        self.stage_instances.remove(&stage_id).await?;
        self.guild_stage_instances
            .remove(&guild_id, &stage_id)
            .await?;

        Ok(())
    }
}
