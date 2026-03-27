use twilight_model::gateway::payload::incoming::VoiceStateUpdate;

use crate::{Error, UpdateCache};

impl UpdateCache for VoiceStateUpdate {
    async fn update(&self, cache: &crate::Cache) -> Result<(), Error> {
        cache.cache_voice_state(self.0.clone()).await?;

        if let (Some(guild_id), Some(member)) = (self.0.guild_id, &self.0.member) {
            cache.cache_member(guild_id, member.clone()).await?;
        }

        Ok(())
    }
}
