use twilight_model::gateway::payload::incoming::{
    GuildScheduledEventCreate, GuildScheduledEventDelete, GuildScheduledEventUpdate,
    GuildScheduledEventUserAdd, GuildScheduledEventUserRemove,
};

use crate::{Error, UpdateCache};

impl UpdateCache for GuildScheduledEventCreate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache
            .cache_guild_scheduled_event(self.guild_id, &self.0)
            .await
    }
}

impl UpdateCache for GuildScheduledEventDelete {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache.scheduled_events.remove(&self.id).await?;
        cache
            .guild_scheduled_events
            .remove(&self.guild_id, &self.id)
            .await?;

        Ok(())
    }
}

impl UpdateCache for GuildScheduledEventUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache
            .cache_guild_scheduled_event(self.guild_id, &self.0)
            .await
    }
}

impl UpdateCache for GuildScheduledEventUserAdd {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        if let Some(mut event) = cache
            .scheduled_events
            .get(&self.guild_scheduled_event_id)
            .await?
        {
            event.user_count = event.user_count.map(|count| count.saturating_add(1));
            cache
                .scheduled_events
                .insert(&self.guild_scheduled_event_id, &event)
                .await?;
        }

        Ok(())
    }
}

impl UpdateCache for GuildScheduledEventUserRemove {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        if let Some(mut event) = cache
            .scheduled_events
            .get(&self.guild_scheduled_event_id)
            .await?
        {
            event.user_count = event.user_count.map(|count| count.saturating_sub(1));
            cache
                .scheduled_events
                .insert(&self.guild_scheduled_event_id, &event)
                .await?;
        }

        Ok(())
    }
}
