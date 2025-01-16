use twilight_model::{
    guild::scheduled_event::GuildScheduledEvent,
    id::{marker::GuildMarker, Id},
};

use crate::Cache;

pub use twilight_model::guild::scheduled_event::GuildScheduledEvent as CachedGuildScheduledEvent;

impl Cache {
    pub(crate) async fn cache_guild_scheduled_events(
        &self,
        guild_id: Id<GuildMarker>,
        guild_scheduled_events: impl IntoIterator<Item = GuildScheduledEvent>,
    ) -> Result<(), crate::Error> {
        for event in guild_scheduled_events {
            self.cache_guild_scheduled_event(guild_id, &event).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_guild_scheduled_event(
        &self,
        guild_id: Id<GuildMarker>,
        guild_scheduled_event: &GuildScheduledEvent,
    ) -> Result<(), crate::Error> {
        self.guild_scheduled_events
            .insert(&guild_id, &guild_scheduled_event.id)
            .await?;

        self.scheduled_events
            .insert(&guild_scheduled_event.id, guild_scheduled_event)
            .await?;

        Ok(())
    }
}
