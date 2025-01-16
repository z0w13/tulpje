use twilight_model::{
    channel::Channel,
    id::{marker::ChannelMarker, Id},
};

use crate::{Cache, Error};

pub use twilight_model::channel::Channel as CachedChannel;

impl Cache {
    pub(crate) async fn cache_channels(
        &self,
        channels: impl IntoIterator<Item = Channel>,
    ) -> Result<(), Error> {
        for channel in channels {
            self.cache_channel(channel).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_channel(&self, channel: Channel) -> Result<(), Error> {
        if let Some(guild_id) = channel.guild_id {
            self.guild_channels.insert(&guild_id, &channel.id).await?;
        }

        self.channels.insert(&channel.id, &channel).await?;
        Ok(())
    }

    pub(crate) async fn delete_channel(&self, channel_id: Id<ChannelMarker>) -> Result<(), Error> {
        if let Some(channel) = self.channels.get(&channel_id).await? {
            if let Some(guild_id) = channel.guild_id {
                self.guild_channels.remove(&guild_id, &channel.id).await?;
            }
        }

        self.channels.remove(&channel_id).await?;
        Ok(())
    }
}
