use twilight_model::gateway::payload::incoming::{
    MemberAdd, MemberChunk, MemberRemove, MemberUpdate,
};

use crate::{Cache, Error, UpdateCache};

impl UpdateCache for MemberAdd {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        if let Some(mut guild) = cache.guilds.get(&self.guild_id).await? {
            guild.member_count = guild.member_count.map(|count| count.saturating_add(1));
            cache.guilds.insert(&guild.id, &guild).await?;
        }

        cache
            .cache_member(self.guild_id, self.member.clone())
            .await?;

        Ok(())
    }
}

impl UpdateCache for MemberChunk {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        if !self.members.is_empty() {
            cache
                .cache_members(self.guild_id, self.members.clone())
                .await?;
        }

        Ok(())
    }
}

impl UpdateCache for MemberRemove {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        if let Some(mut guild) = cache.guilds.get(&self.guild_id).await? {
            guild.member_count = guild.member_count.map(|count| count.saturating_sub(1));
            cache.guilds.insert(&guild.id, &guild).await?;
        }

        cache.members.remove(&(self.guild_id, self.user.id)).await?;
        cache
            .guild_members
            .remove(&self.guild_id, &self.user.id)
            .await?;

        cache
            .user_guilds
            .remove(&self.user.id, &self.guild_id)
            .await?;

        // user is in no tracked guils, we can remove them from the cache completely
        if !cache.user_guilds.is_empty(&self.user.id).await? {
            cache.users.remove(&self.user.id).await?;
        }

        Ok(())
    }
}

impl UpdateCache for MemberUpdate {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        let key = (self.guild_id, self.user.id);
        if let Some(mut member) = cache.members.get(&key).await? {
            member.update_with_member_update(self);
            cache.members.insert(&key, &member).await?;
        }

        Ok(())
    }
}
