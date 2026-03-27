pub(crate) mod channel;
pub(crate) mod emoji;
pub(crate) mod guild;
pub(crate) mod guild_scheduled_event;
pub(crate) mod integration;
pub(crate) mod interaction;
pub(crate) mod member;
pub(crate) mod message;
pub(crate) mod presence;
pub(crate) mod reaction;
pub(crate) mod role;
pub(crate) mod stage_instance;
pub(crate) mod sticker;
pub(crate) mod thread;
pub(crate) mod voice_state;

use twilight_model::gateway::payload::incoming::{Ready, UserUpdate};

use crate::UpdateCache;

impl UpdateCache for Ready {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.cache_current_user(&self.user).await?;

        for guild in &self.guilds {
            cache.unavailable_guild(guild.id).await?;
        }

        Ok(())
    }
}

impl UpdateCache for UserUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), crate::Error> {
        cache.cache_current_user(&self.0).await?;

        Ok(())
    }
}
