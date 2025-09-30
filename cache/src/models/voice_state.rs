use serde::{Deserialize, Serialize};
use twilight_model::{
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, UserMarker},
    },
    util::Timestamp,
    voice::VoiceState,
};

use crate::{Cache, Error};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachedVoiceState {
    channel_id: Id<ChannelMarker>,
    deaf: bool,
    guild_id: Id<GuildMarker>,
    mute: bool,
    request_to_speak_timestamp: Option<Timestamp>,
    self_deaf: bool,
    self_mute: bool,
    self_stream: bool,
    self_video: bool,
    session_id: String,
    suppress: bool,
    user_id: Id<UserMarker>,
}

impl From<(Id<ChannelMarker>, Id<GuildMarker>, VoiceState)> for CachedVoiceState {
    fn from(
        (channel_id, guild_id, voice_state): (Id<ChannelMarker>, Id<GuildMarker>, VoiceState),
    ) -> Self {
        // Reasons for dropping fields:
        //
        // - `channel_id`: provided as a function parameter
        // - `guild_id`: provided as a function parameter
        // - `member`: we have the user's ID from the `user_id` field
        #[expect(
            clippy::unneeded_field_pattern,
            reason = "clearer that we're explicitly skipping those fields"
        )]
        let VoiceState {
            channel_id: _,
            deaf,
            guild_id: _,
            member: _,
            mute,
            self_deaf,
            self_mute,
            self_stream,
            self_video,
            session_id,
            suppress,
            user_id,
            request_to_speak_timestamp,
        } = voice_state;

        Self {
            channel_id,
            deaf,
            guild_id,
            mute,
            request_to_speak_timestamp,
            self_deaf,
            self_mute,
            self_stream,
            self_video,
            session_id,
            suppress,
            user_id,
        }
    }
}

impl PartialEq<VoiceState> for CachedVoiceState {
    fn eq(&self, other: &VoiceState) -> bool {
        Some(self.channel_id) == other.channel_id
            && self.deaf == other.deaf
            && Some(self.guild_id) == other.guild_id
            && self.mute == other.mute
            && self.request_to_speak_timestamp == other.request_to_speak_timestamp
            && self.self_deaf == other.self_deaf
            && self.self_mute == other.self_mute
            && self.self_stream == other.self_stream
            && self.self_video == other.self_video
            && self.session_id == other.session_id
            && self.suppress == other.suppress
            && self.user_id == other.user_id
    }
}

impl Cache {
    pub(crate) async fn cache_voice_states(
        &self,
        voice_states: impl IntoIterator<Item = VoiceState>,
    ) -> Result<(), Error> {
        for voice_state in voice_states {
            self.cache_voice_state(voice_state).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_voice_state(&self, voice_state: VoiceState) -> Result<(), Error> {
        // This should always exist, but let's check just in case.
        let Some(guild_id) = voice_state.guild_id else {
            return Ok(());
        };

        let user_id = voice_state.user_id;

        // Check if the user is switching channels in the same guild (ie. they already have a voice state entry)
        if let Some(voice_state) = self.voice_states.get(&(guild_id, user_id)).await? {
            self.voice_state_channels
                .remove(&voice_state.channel_id, &(guild_id, user_id))
                .await?;
        }

        if let Some(channel_id) = voice_state.channel_id {
            let cached_voice_state = CachedVoiceState::from((channel_id, guild_id, voice_state));

            self.voice_states
                .insert(&(guild_id, user_id), &cached_voice_state)
                .await?;
            self.voice_state_guilds.insert(&guild_id, &user_id).await?;
            self.voice_state_channels
                .insert(&channel_id, &(guild_id, user_id))
                .await?;
        } else {
            // voice channel_id does not exist, signifying that the user has left

            self.voice_state_guilds.remove(&guild_id, &user_id).await?;
            self.voice_states.remove(&(guild_id, user_id)).await?;
        }

        Ok(())
    }
}
