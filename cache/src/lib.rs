mod config;
mod event;
mod repository;

pub mod models;

use std::{
    collections::VecDeque,
    hash::{DefaultHasher, Hash, Hasher as _},
    ops::Deref as _,
};

use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use twilight_model::{
    gateway::event::Event,
    id::{
        marker::{
            ChannelMarker, EmojiMarker, GuildMarker, IntegrationMarker, MessageMarker, RoleMarker,
            ScheduledEventMarker, StageMarker, StickerMarker, UserMarker,
        },
        Id,
    },
};

use models::{
    channel::CachedChannel,
    emoji::CachedEmoji,
    guild::CachedGuild,
    guild_scheduled_event::CachedGuildScheduledEvent,
    integration::CachedGuildIntegration,
    member::CachedMember,
    message::CachedMessage,
    presence::CachedPresence,
    role::CachedRole,
    stage_instance::CachedStageInstance,
    sticker::CachedSticker,
    user::{CachedCurrentUser, CachedUser},
    voice_state::CachedVoiceState,
};
use repository::{MappedSetRepository, Repository, SetRepository, SingleRepository};

pub use config::Config;
pub use twilight_cache_inmemory::Config as TwilightConfig;
pub use twilight_cache_inmemory::ResourceType;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Cache {
    pub config: Config,

    pub guilds: Repository<Id<GuildMarker>, CachedGuild>,
    pub guild_channels: MappedSetRepository<Id<GuildMarker>, Id<ChannelMarker>>,
    pub guild_scheduled_events: MappedSetRepository<Id<GuildMarker>, Id<ScheduledEventMarker>>,
    pub guild_integrations: MappedSetRepository<Id<GuildMarker>, Id<IntegrationMarker>>,
    pub guild_members: MappedSetRepository<Id<GuildMarker>, Id<UserMarker>>,
    pub guild_presences: MappedSetRepository<Id<GuildMarker>, Id<UserMarker>>,
    pub guild_emojis: MappedSetRepository<Id<GuildMarker>, Id<EmojiMarker>>,
    pub guild_roles: MappedSetRepository<Id<GuildMarker>, Id<RoleMarker>>,
    pub guild_stage_instances: MappedSetRepository<Id<GuildMarker>, Id<StageMarker>>,
    pub guild_stickers: MappedSetRepository<Id<GuildMarker>, Id<StickerMarker>>,
    pub unavailable_guilds: SetRepository<Id<GuildMarker>>,

    pub channels: Repository<Id<ChannelMarker>, CachedChannel>,
    pub channel_messages: Repository<Id<ChannelMarker>, VecDeque<Id<MessageMarker>>>,

    pub scheduled_events: Repository<Id<ScheduledEventMarker>, CachedGuildScheduledEvent>,
    pub integrations:
        Repository<(Id<GuildMarker>, Id<IntegrationMarker>), GuildResource<CachedGuildIntegration>>,
    pub members: Repository<(Id<GuildMarker>, Id<UserMarker>), CachedMember>,
    pub messages: Repository<Id<MessageMarker>, CachedMessage>,
    pub presences: Repository<(Id<GuildMarker>, Id<UserMarker>), CachedPresence>,
    pub emojis: Repository<Id<EmojiMarker>, GuildResource<CachedEmoji>>,
    pub roles: Repository<Id<RoleMarker>, GuildResource<CachedRole>>,
    pub stage_instances: Repository<Id<StageMarker>, GuildResource<CachedStageInstance>>,
    pub stickers: Repository<Id<StickerMarker>, GuildResource<CachedSticker>>,

    pub current_user: SingleRepository<CachedCurrentUser>,
    pub users: Repository<Id<UserMarker>, CachedUser>,
    pub user_guilds: MappedSetRepository<Id<UserMarker>, Id<GuildMarker>>,

    pub voice_state_channels:
        MappedSetRepository<Id<ChannelMarker>, (Id<GuildMarker>, Id<UserMarker>)>,
    pub voice_state_guilds: MappedSetRepository<Id<GuildMarker>, Id<UserMarker>>,
    pub voice_states: Repository<(Id<GuildMarker>, Id<UserMarker>), CachedVoiceState>,
}

pub(crate) fn hash<T: Hash>(val: T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

impl Cache {
    pub fn new(redis: ConnectionManager, config: Config) -> Self {
        Self {
            guilds: Repository::new("guilds", config.wants(ResourceType::GUILD), redis.clone()),
            guild_channels: MappedSetRepository::new(
                "guild_channels",
                config.wants(ResourceType::CHANNEL),
                redis.clone(),
            ),
            guild_scheduled_events: MappedSetRepository::new(
                "guild_scheduled_events",
                config
                    .resource_types
                    .contains(ResourceType::GUILD_SCHEDULED_EVENT),
                redis.clone(),
            ),
            guild_integrations: MappedSetRepository::new(
                "guild_integrations",
                config.wants(ResourceType::INTEGRATION),
                redis.clone(),
            ),
            guild_members: MappedSetRepository::new(
                "guild_members",
                config.wants(ResourceType::MEMBER),
                redis.clone(),
            ),
            guild_presences: MappedSetRepository::new(
                "guild_presences",
                config.wants(ResourceType::PRESENCE),
                redis.clone(),
            ),
            guild_emojis: MappedSetRepository::new(
                "guild_emojis",
                config.wants(ResourceType::EMOJI),
                redis.clone(),
            ),
            guild_roles: MappedSetRepository::new(
                "guild_roles",
                config.wants(ResourceType::ROLE),
                redis.clone(),
            ),
            guild_stage_instances: MappedSetRepository::new(
                "guild_stage_instances",
                config.wants(ResourceType::STAGE_INSTANCE),
                redis.clone(),
            ),
            guild_stickers: MappedSetRepository::new(
                "guild_stickers",
                config.wants(ResourceType::STICKER),
                redis.clone(),
            ),
            unavailable_guilds: SetRepository::new(
                "unavailable_guilds",
                config.wants(ResourceType::GUILD),
                redis.clone(),
            ),

            channels: Repository::new(
                "channels",
                config.wants(ResourceType::CHANNEL),
                redis.clone(),
            ),
            channel_messages: Repository::new(
                "channel_messages",
                config.wants(ResourceType::MESSAGE),
                redis.clone(),
            ),

            scheduled_events: Repository::new(
                "scheduled_events",
                config
                    .resource_types
                    .contains(ResourceType::GUILD_SCHEDULED_EVENT),
                redis.clone(),
            ),
            integrations: Repository::new(
                "integrations",
                config.wants(ResourceType::INTEGRATION),
                redis.clone(),
            ),
            members: Repository::new("members", config.wants(ResourceType::MEMBER), redis.clone()),
            messages: Repository::new(
                "messages",
                config.wants(ResourceType::MESSAGE),
                redis.clone(),
            ),
            presences: Repository::new(
                "presences",
                config.wants(ResourceType::PRESENCE),
                redis.clone(),
            ),
            emojis: Repository::new("emojis", config.wants(ResourceType::EMOJI), redis.clone()),

            roles: Repository::new("roles", config.wants(ResourceType::ROLE), redis.clone()),
            stage_instances: Repository::new(
                "stage_instances",
                config.wants(ResourceType::STAGE_INSTANCE),
                redis.clone(),
            ),
            stickers: Repository::new(
                "stickers",
                config.wants(ResourceType::STICKER),
                redis.clone(),
            ),

            current_user: SingleRepository::new(
                "current_user",
                config.wants(ResourceType::USER_CURRENT),
                redis.clone(),
            ),
            users: Repository::new("emojis", config.wants(ResourceType::USER), redis.clone()),
            user_guilds: MappedSetRepository::new(
                "user_guilds",
                config.wants(ResourceType::USER),
                redis.clone(),
            ),

            voice_state_channels: MappedSetRepository::new(
                "voice_state_channels",
                config.wants(ResourceType::VOICE_STATE),
                redis.clone(),
            ),
            voice_state_guilds: MappedSetRepository::new(
                "voice_state_guilds",
                config.wants(ResourceType::VOICE_STATE),
                redis.clone(),
            ),
            voice_states: Repository::new(
                "voice_states",
                config.wants(ResourceType::VOICE_STATE),
                redis,
            ),

            config,
        }
    }

    pub async fn update(&self, event: &impl UpdateCache) -> Result<(), Error> {
        event.update(self).await
    }
}

impl UpdateCache for Event {
    async fn update(&self, cache: &Cache) -> Result<(), Error> {
        #[expect(clippy::use_self, reason = "it's clearer to refer to Event")]
        match self {
            Event::ChannelCreate(v) => cache.update(v.deref()).await,
            Event::ChannelDelete(v) => cache.update(v.deref()).await,
            Event::ChannelPinsUpdate(v) => cache.update(v).await,
            Event::ChannelUpdate(v) => cache.update(v.deref()).await,
            Event::GuildCreate(v) => cache.update(v.deref()).await,
            Event::GuildDelete(v) => cache.update(v).await,
            Event::GuildEmojisUpdate(v) => cache.update(v).await,
            Event::GuildStickersUpdate(v) => cache.update(v).await,
            Event::GuildUpdate(v) => cache.update(v.deref()).await,
            Event::GuildScheduledEventCreate(v) => cache.update(v.deref()).await,
            Event::GuildScheduledEventDelete(v) => cache.update(v.deref()).await,
            Event::GuildScheduledEventUpdate(v) => cache.update(v.deref()).await,
            Event::GuildScheduledEventUserAdd(v) => cache.update(v).await,
            Event::GuildScheduledEventUserRemove(v) => cache.update(v).await,
            Event::IntegrationCreate(v) => cache.update(v.deref()).await,
            Event::IntegrationDelete(v) => cache.update(v).await,
            Event::IntegrationUpdate(v) => cache.update(v.deref()).await,
            Event::InteractionCreate(v) => cache.update(v.deref()).await,
            Event::MemberAdd(v) => cache.update(v.deref()).await,
            Event::MemberRemove(v) => cache.update(v).await,
            Event::MemberUpdate(v) => cache.update(v.deref()).await,
            Event::MemberChunk(v) => cache.update(v).await,
            Event::MessageCreate(v) => cache.update(v.deref()).await,
            Event::MessageDelete(v) => cache.update(v).await,
            Event::MessageDeleteBulk(v) => cache.update(v).await,
            Event::MessageUpdate(v) => cache.update(v.deref()).await,
            Event::PresenceUpdate(v) => cache.update(v.deref()).await,
            Event::ReactionAdd(v) => cache.update(v.deref()).await,
            Event::ReactionRemove(v) => cache.update(v.deref()).await,
            Event::ReactionRemoveAll(v) => cache.update(v).await,
            Event::ReactionRemoveEmoji(v) => cache.update(v).await,
            Event::Ready(v) => cache.update(v.deref()).await,
            Event::RoleCreate(v) => cache.update(v).await,
            Event::RoleDelete(v) => cache.update(v).await,
            Event::RoleUpdate(v) => cache.update(v).await,
            Event::StageInstanceCreate(v) => cache.update(v).await,
            Event::StageInstanceDelete(v) => cache.update(v).await,
            Event::StageInstanceUpdate(v) => cache.update(v).await,
            Event::ThreadCreate(v) => cache.update(v.deref()).await,
            Event::ThreadUpdate(v) => cache.update(v.deref()).await,
            Event::ThreadDelete(v) => cache.update(v).await,
            Event::ThreadListSync(v) => cache.update(v).await,
            Event::UnavailableGuild(v) => cache.update(v).await,
            Event::UserUpdate(v) => cache.update(v).await,
            Event::VoiceStateUpdate(v) => cache.update(v.deref()).await,
            // Ignored events.
            Event::AutoModerationActionExecution(_)
            | Event::AutoModerationRuleCreate(_)
            | Event::AutoModerationRuleDelete(_)
            | Event::AutoModerationRuleUpdate(_)
            | Event::BanAdd(_)
            | Event::BanRemove(_)
            | Event::CommandPermissionsUpdate(_)
            | Event::GatewayClose(_)
            | Event::GatewayHeartbeat
            | Event::GatewayHeartbeatAck
            | Event::GatewayHello(_)
            | Event::GatewayInvalidateSession(_)
            | Event::GatewayReconnect
            | Event::GuildAuditLogEntryCreate(_)
            | Event::GuildIntegrationsUpdate(_)
            | Event::InviteCreate(_)
            | Event::InviteDelete(_)
            | Event::Resumed
            | Event::ThreadMembersUpdate(_)
            | Event::ThreadMemberUpdate(_)
            | Event::TypingStart(_)
            | Event::VoiceServerUpdate(_)
            | Event::WebhooksUpdate(_) => Ok(()),
            _ => Ok(()), // TODO: Remove this once we've implemented all events
        }
    }
}

pub trait UpdateCache {
    #[expect(async_fn_in_trait, reason = "honestly, what else do we do")]
    async fn update(&self, cache: &Cache) -> Result<(), Error>;
}

#[derive(Serialize, Deserialize)]
pub struct GuildResource<T> {
    guild_id: Id<GuildMarker>,
    value: T,
}
