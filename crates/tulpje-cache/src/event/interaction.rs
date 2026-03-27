use twilight_model::{
    application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate,
};

use crate::{Error, UpdateCache};

impl UpdateCache for InteractionCreate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        // Cache interaction member
        if let Some(member) = &self.member
            && let Some(guild_id) = self.guild_id
            && let Some(user) = &member.user
        {
            cache.cache_user(user, self.guild_id).await?;

            cache
                .cache_borrowed_partial_member(guild_id, member, user.id)
                .await?;
        }

        // Cache interaction user
        if let Some(user) = &self.user {
            cache.cache_user(user, None).await?;
        }

        // Cache resolved interaction data
        if let Some(InteractionData::ApplicationCommand(data)) = &self.data
            && let Some(resolved) = &data.resolved
        {
            // Cache resolved users and members
            for user in resolved.users.values() {
                cache.cache_user(user, self.guild_id).await?;

                // member should always match, because resolved members
                // are guaranteed to have a matching resolved user
                if let Some(guild_id) = self.guild_id
                    && let Some(member) = resolved.members.get(&user.id)
                {
                    cache
                        .cache_borrowed_interaction_member(guild_id, member, user.id)
                        .await?;
                }
            }

            // Cache resolved roles
            if let Some(guild_id) = self.guild_id {
                cache
                    .cache_roles(guild_id, resolved.roles.values().cloned())
                    .await?;
            }
        }

        Ok(())
    }
}
