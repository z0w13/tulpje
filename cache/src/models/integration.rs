use twilight_model::{
    guild::GuildIntegration,
    id::{
        marker::{GuildMarker, IntegrationMarker},
        Id,
    },
};

use crate::{Cache, Error, GuildResource};

pub use twilight_model::guild::GuildIntegration as CachedGuildIntegration;

impl Cache {
    pub(crate) async fn cache_integration(
        &self,
        guild_id: Id<GuildMarker>,
        integration: &GuildIntegration,
    ) -> Result<(), Error> {
        self.guild_integrations
            .insert(&guild_id, &integration.id)
            .await?;

        self.integrations
            .insert(
                &(guild_id, integration.id),
                &GuildResource {
                    guild_id,
                    value: integration.clone(),
                },
            )
            .await?;

        Ok(())
    }

    pub(crate) async fn delete_integration(
        &self,
        guild_id: Id<GuildMarker>,
        integration_id: Id<IntegrationMarker>,
    ) -> Result<(), Error> {
        self.integrations
            .remove(&(guild_id, integration_id))
            .await?;

        self.guild_integrations
            .remove(&guild_id, &integration_id)
            .await?;

        Ok(())
    }
}
