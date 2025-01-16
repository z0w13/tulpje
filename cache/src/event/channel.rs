use twilight_model::gateway::payload::incoming::{
    ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate,
};

use crate::UpdateCache;

impl UpdateCache for ChannelCreate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.cache_channel(self.0.clone()).await
    }
}

impl UpdateCache for ChannelDelete {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.delete_channel(self.id).await
    }
}

impl UpdateCache for ChannelPinsUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        if let Some(mut channel) = cache.channels.get(&self.channel_id).await? {
            channel.last_pin_timestamp = self.last_pin_timestamp;
            cache.channels.insert(&channel.id, &channel).await?;
        }

        Ok(())
    }
}

impl UpdateCache for ChannelUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.cache_channel(self.0.clone()).await
    }
}
