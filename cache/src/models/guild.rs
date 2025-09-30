use std::{hash::Hash, mem};

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use twilight_model::{
    gateway::payload::incoming::GuildUpdate,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, Guild, GuildFeature,
        MfaLevel, NSFWLevel, Permissions, PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::{
        Id,
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
    },
    util::{ImageHash, Timestamp},
};

use crate::{
    Cache, Error,
    repository::{MappedSetRepository, Repository},
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachedGuild {
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: AfkTimeout,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub banner: Option<ImageHash>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<ImageHash>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<GuildFeature>,
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub joined_at: Option<Timestamp>,
    pub large: bool,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub max_video_channel_users: Option<u64>,
    pub member_count: Option<u64>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub nsfw_level: NSFWLevel,
    pub owner_id: Id<UserMarker>,
    pub owner: Option<bool>,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: PremiumTier,
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
    pub splash: Option<ImageHash>,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub system_channel_flags: SystemChannelFlags,
    pub unavailable: Option<bool>,
    pub vanity_url_code: Option<String>,
    pub verification_level: VerificationLevel,
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    pub widget_enabled: Option<bool>,
}

impl CachedGuild {
    pub(crate) fn update_with_guild_update(&mut self, guild_update: &GuildUpdate) {
        self.afk_channel_id = guild_update.afk_channel_id;
        self.afk_timeout = guild_update.afk_timeout;
        self.banner = guild_update.banner;
        self.default_message_notifications = guild_update.default_message_notifications;
        self.description = guild_update.description.clone();
        self.features = guild_update.features.clone();
        self.icon = guild_update.icon;
        self.max_members = guild_update.max_members;
        self.max_presences = Some(guild_update.max_presences.unwrap_or(25000));
        self.mfa_level = guild_update.mfa_level;
        self.name = guild_update.name.clone();
        self.nsfw_level = guild_update.nsfw_level;
        self.owner = guild_update.owner;
        self.owner_id = guild_update.owner_id;
        self.permissions = guild_update.permissions;
        self.preferred_locale = guild_update.preferred_locale.clone();
        self.premium_tier = guild_update.premium_tier;
        self.premium_subscription_count
            .replace(guild_update.premium_subscription_count.unwrap_or_default());
        self.splash = guild_update.splash;
        self.system_channel_id = guild_update.system_channel_id;
        self.verification_level = guild_update.verification_level;
        self.vanity_url_code = guild_update.vanity_url_code.clone();
        self.widget_channel_id = guild_update.widget_channel_id;
        self.widget_enabled = guild_update.widget_enabled;
    }
}

impl From<Guild> for CachedGuild {
    fn from(guild: Guild) -> Self {
        let Guild {
            afk_channel_id,
            afk_timeout,
            application_id,
            banner,
            default_message_notifications,
            description,
            discovery_splash,
            explicit_content_filter,
            features,
            icon,
            id,
            joined_at,
            large,
            max_members,
            max_presences,
            max_video_channel_users,
            member_count,
            mfa_level,
            name,
            nsfw_level,
            owner_id,
            owner,
            permissions,
            preferred_locale,
            premium_progress_bar_enabled,
            premium_subscription_count,
            premium_tier,
            public_updates_channel_id,
            rules_channel_id,
            safety_alerts_channel_id,
            splash,
            system_channel_flags,
            system_channel_id,
            unavailable,
            vanity_url_code,
            verification_level,
            widget_channel_id,
            widget_enabled,
            ..
        } = guild;

        Self {
            afk_channel_id,
            afk_timeout,
            application_id,
            banner,
            default_message_notifications,
            description,
            discovery_splash,
            explicit_content_filter,
            features,
            icon,
            id,
            joined_at,
            large,
            max_members,
            max_presences,
            max_video_channel_users,
            member_count,
            mfa_level,
            name,
            nsfw_level,
            owner_id,
            owner,
            permissions,
            preferred_locale,
            premium_progress_bar_enabled,
            premium_subscription_count,
            premium_tier,
            public_updates_channel_id,
            rules_channel_id,
            safety_alerts_channel_id,
            splash,
            system_channel_id,
            system_channel_flags,
            unavailable,
            vanity_url_code,
            verification_level,
            widget_channel_id,
            widget_enabled,
        }
    }
}

impl PartialEq<Guild> for CachedGuild {
    fn eq(&self, other: &Guild) -> bool {
        self.afk_channel_id == other.afk_channel_id
            && self.afk_timeout == other.afk_timeout
            && self.application_id == other.application_id
            && self.banner == other.banner
            && self.default_message_notifications == other.default_message_notifications
            && self.description == other.description
            && self.discovery_splash == other.discovery_splash
            && self.explicit_content_filter == other.explicit_content_filter
            && self.features == other.features
            && self.icon == other.icon
            && self.joined_at == other.joined_at
            && self.large == other.large
            && self.max_members == other.max_members
            && self.max_presences == other.max_presences
            && self.max_video_channel_users == other.max_video_channel_users
            && self.member_count == other.member_count
            && self.mfa_level == other.mfa_level
            && self.name == other.name
            && self.nsfw_level == other.nsfw_level
            && self.owner_id == other.owner_id
            && self.owner == other.owner
            && self.permissions == other.permissions
            && self.preferred_locale == other.preferred_locale
            && self.premium_progress_bar_enabled == other.premium_progress_bar_enabled
            && self.premium_subscription_count == other.premium_subscription_count
            && self.premium_tier == other.premium_tier
            && self.public_updates_channel_id == other.public_updates_channel_id
            && self.rules_channel_id == other.rules_channel_id
            && self.safety_alerts_channel_id == other.safety_alerts_channel_id
            && self.splash == other.splash
            && self.system_channel_id == other.system_channel_id
            && self.system_channel_flags == other.system_channel_flags
            && self.unavailable == other.unavailable
            && self.vanity_url_code == other.vanity_url_code
            && self.verification_level == other.verification_level
            && self.widget_channel_id == other.widget_channel_id
            && self.widget_enabled == other.widget_enabled
    }
}

impl Cache {
    pub(crate) async fn unavailable_guild(&self, guild_id: Id<GuildMarker>) -> Result<(), Error> {
        self.unavailable_guilds.insert(&guild_id).await?;
        self.delete_guild(guild_id, true).await?;

        Ok(())
    }

    pub(crate) async fn cache_guild(&self, mut guild: Guild) -> Result<(), Error> {
        for channel in &mut guild.channels {
            channel.guild_id = Some(guild.id);
        }
        for thread in &mut guild.threads {
            thread.guild_id = Some(guild.id);
        }

        self.cache_channels(mem::take(&mut guild.channels)).await?;
        self.cache_channels(mem::take(&mut guild.threads)).await?;
        self.cache_emojis(guild.id, mem::take(&mut guild.emojis))
            .await?;
        self.cache_members(guild.id, mem::take(&mut guild.members))
            .await?;
        self.cache_presences(guild.id, mem::take(&mut guild.presences))
            .await?;
        self.cache_roles(guild.id, mem::take(&mut guild.roles))
            .await?;
        self.cache_stickers(guild.id, mem::take(&mut guild.stickers))
            .await?;
        self.cache_voice_states(mem::take(&mut guild.voice_states))
            .await?;
        self.cache_stage_instances(guild.id, mem::take(&mut guild.stage_instances))
            .await?;
        self.cache_guild_scheduled_events(guild.id, mem::take(&mut guild.guild_scheduled_events))
            .await?;

        self.unavailable_guilds.remove(&guild.id).await?;
        self.guilds
            .insert(&guild.id, &CachedGuild::from(guild.clone()))
            .await?;

        Ok(())
    }

    pub(crate) async fn delete_guild(
        &self,
        guild_id: Id<GuildMarker>,
        unavailable: bool,
    ) -> Result<(), Error> {
        async fn remove_ids<
            T: Eq + Hash + Serialize + DeserializeOwned,
            U: Serialize + DeserializeOwned,
        >(
            guild_map: &MappedSetRepository<Id<GuildMarker>, T>,
            container: &Repository<T, U>,
            guild_id: Id<GuildMarker>,
        ) -> Result<(), Error> {
            let ids = guild_map.members(&guild_id).await?;
            container.remove_multi(&ids).await?;
            Ok(())
        }

        if unavailable {
            if let Some(mut guild) = self.guilds.get(&guild_id).await? {
                guild.unavailable = Some(true);
                self.guilds.insert(&guild_id, &guild).await?;
            }
        } else {
            self.guilds.remove(&guild_id).await?;
        }

        remove_ids(&self.guild_channels, &self.channels, guild_id).await?;
        remove_ids(&self.guild_emojis, &self.emojis, guild_id).await?;
        remove_ids(&self.guild_roles, &self.roles, guild_id).await?;
        remove_ids(&self.guild_stickers, &self.stickers, guild_id).await?;

        self.voice_state_guilds.clear(&guild_id).await?;

        let members_to_remove: Vec<_> = self
            .guild_members
            .members(&guild_id)
            .await?
            .into_iter()
            .map(|user_id| (guild_id, user_id))
            .collect();
        self.members.remove_multi(members_to_remove.iter()).await?;
        self.guild_members.clear(&guild_id).await?;

        let presences_to_remove: Vec<_> = self
            .guild_presences
            .members(&guild_id)
            .await?
            .into_iter()
            .map(|user_id| (guild_id, user_id))
            .collect();
        self.presences
            .remove_multi(presences_to_remove.iter())
            .await?;
        self.guild_presences.clear(&guild_id).await?;

        Ok(())
    }

    pub(crate) async fn update_guild(&self, guild_update: &GuildUpdate) -> Result<(), Error> {
        let Some(mut guild) = self.guilds.get(&guild_update.id).await? else {
            return Ok(());
        };

        guild.update_with_guild_update(guild_update);
        self.guilds.insert(&guild.id, &guild).await?;

        Ok(())
    }
}
