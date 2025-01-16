use twilight_model::gateway::payload::incoming::{
    MessageCreate, MessageDelete, MessageDeleteBulk, MessageUpdate,
};

use crate::{models::message::CachedMessage, Cache, Error, UpdateCache};

impl UpdateCache for MessageCreate {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        cache.cache_user(&self.author, self.guild_id).await?;

        if let (Some(member), Some(guild_id)) = (&self.member, self.guild_id) {
            cache
                .cache_borrowed_partial_member(guild_id, member, self.author.id)
                .await?;
        }

        let mut channel_messages = cache
            .channel_messages
            .get(&self.channel_id)
            .await?
            .unwrap_or_default();

        if channel_messages.len() >= cache.config.message_cache_size {
            if let Some(popped_id) = channel_messages.pop_back() {
                cache.messages.remove(&popped_id).await?;
            }
        }
        channel_messages.push_front(self.id);

        cache
            .channel_messages
            .insert(&self.channel_id, &channel_messages)
            .await?;
        cache
            .messages
            .insert(&self.id, &CachedMessage::from(self.0.clone()))
            .await?;

        Ok(())
    }
}

impl UpdateCache for MessageDelete {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        cache.messages.remove(&self.id).await?;

        let mut channel_messages = cache
            .channel_messages
            .get(&self.channel_id)
            .await?
            .unwrap_or_default();

        if let Some(idx) = channel_messages.iter().position(|id| *id == self.id) {
            channel_messages.remove(idx);
            cache
                .channel_messages
                .insert(&self.channel_id, &channel_messages)
                .await?;
        }

        Ok(())
    }
}

impl UpdateCache for MessageDeleteBulk {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        let mut channel_messages = cache
            .channel_messages
            .get(&self.channel_id)
            .await?
            .unwrap_or_default();

        cache.messages.remove_multi(&self.ids).await?;
        for id in &self.ids {
            if let Some(idx) = channel_messages
                .iter()
                .position(|message_id| message_id == id)
            {
                channel_messages.remove(idx);
            }
        }

        cache
            .channel_messages
            .insert(&self.channel_id, &channel_messages)
            .await?;

        Ok(())
    }
}

impl UpdateCache for MessageUpdate {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        cache.cache_user(&self.author, self.guild_id).await?;

        if let (Some(member), Some(guild_id)) = (&self.member, self.guild_id) {
            cache
                .cache_borrowed_partial_member(guild_id, member, self.author.id)
                .await?;
        }

        // if the message was still in the cache, there's nothing to do after
        // updating it
        if !cache
            .messages
            .insert(&self.id, &CachedMessage::from(self.0.clone()))
            .await?
        {
            return Ok(());
        }

        let mut channel_messages = cache
            .channel_messages
            .get(&self.channel_id)
            .await?
            .unwrap_or_default();

        if channel_messages.len() >= cache.config.message_cache_size {
            if let Some(popped_id) = channel_messages.pop_back() {
                cache.messages.remove(&popped_id).await?;
            }
        }
        channel_messages.push_front(self.id);

        cache
            .channel_messages
            .insert(&self.channel_id, &channel_messages)
            .await?;

        Ok(())
    }
}
