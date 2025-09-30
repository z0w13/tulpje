use serde::{Deserialize, Serialize};
use twilight_model::{
    application::interaction::InteractionType,
    channel::{
        Attachment, ChannelMention, Message,
        message::{
            Component, Embed, MessageActivity, MessageApplication, MessageCall, MessageFlags,
            MessageInteraction, MessageReference, MessageSnapshot, MessageSticker, MessageType,
            Reaction, RoleSubscriptionData,
        },
    },
    guild::PartialMember,
    id::{
        Id,
        marker::{
            ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, MessageMarker,
            RoleMarker, UserMarker, WebhookMarker,
        },
    },
    poll::Poll,
    util::Timestamp,
};

/// Information about the message interaction.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachedMessageInteraction {
    pub id: Id<InteractionMarker>,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub name: String,
    pub user_id: Id<UserMarker>,
}

impl From<MessageInteraction> for CachedMessageInteraction {
    fn from(message_interaction: MessageInteraction) -> Self {
        // Reasons for dropping fields:
        //
        // - `member`: we have the user's ID from the `user_id` field
        #[expect(
            clippy::unneeded_field_pattern,
            reason = "clearer that we're explicitly skipping those fields"
        )]
        let MessageInteraction {
            id,
            kind,
            member: _,
            name,
            user,
        } = message_interaction;

        Self {
            id,
            kind,
            name,
            user_id: user.id,
        }
    }
}

impl PartialEq<MessageInteraction> for CachedMessageInteraction {
    fn eq(&self, other: &MessageInteraction) -> bool {
        self.id == other.id
            && self.kind == other.kind
            && self.name == other.name
            && self.user_id == other.user.id
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CachedMessage {
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub attachments: Vec<Attachment>,
    pub author: Id<UserMarker>,
    pub call: Option<MessageCall>,
    pub channel_id: Id<ChannelMarker>,
    pub components: Vec<Component>,
    pub content: String,
    pub edited_timestamp: Option<Timestamp>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub id: Id<MessageMarker>,
    pub interaction: Option<CachedMessageInteraction>,
    pub kind: MessageType,
    pub member: Option<PartialMember>,
    pub mention_channels: Vec<ChannelMention>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<Id<RoleMarker>>,
    pub mentions: Vec<Id<UserMarker>>,
    pub message_snapshots: Vec<MessageSnapshot>,
    pub pinned: bool,
    pub poll: Option<Poll>,
    pub reactions: Vec<Reaction>,
    pub reference: Option<MessageReference>,
    pub role_subscription_data: Option<RoleSubscriptionData>,
    pub sticker_items: Vec<MessageSticker>,
    pub thread_id: Option<Id<ChannelMarker>>,
    pub timestamp: Timestamp,
    pub tts: bool,
    pub webhook_id: Option<Id<WebhookMarker>>,
}

impl From<Message> for CachedMessage {
    #[expect(deprecated)]
    fn from(message: Message) -> Self {
        #[expect(
            clippy::unneeded_field_pattern,
            reason = "clearer that we're explicitly skipping those fields"
        )]
        let Message {
            activity,
            application,
            application_id,
            attachments,
            author,
            call,
            channel_id,
            components,
            content,
            edited_timestamp,
            embeds,
            flags,
            guild_id,
            id,
            interaction,
            interaction_metadata: _,
            kind,
            member,
            mention_channels,
            mention_everyone,
            mention_roles,
            mentions,
            message_snapshots,
            pinned,
            poll,
            reactions,
            reference,
            referenced_message: _,
            role_subscription_data,
            sticker_items,
            timestamp,
            thread,
            tts,
            webhook_id,
        } = message;

        Self {
            id,
            activity,
            application,
            application_id,
            attachments,
            author: author.id,
            call,
            channel_id,
            components,
            content,
            edited_timestamp,
            embeds,
            flags,
            guild_id,
            interaction: interaction.map(CachedMessageInteraction::from),
            kind,
            member,
            mention_channels,
            mention_everyone,
            mention_roles,
            mentions: mentions.into_iter().map(|mention| mention.id).collect(),
            message_snapshots,
            pinned,
            poll,
            reactions,
            reference,
            role_subscription_data,
            sticker_items,
            thread_id: thread.map(|thread| thread.id),
            timestamp,
            tts,
            webhook_id,
        }
    }
}

impl PartialEq<Message> for CachedMessage {
    #[expect(deprecated)]
    fn eq(&self, other: &Message) -> bool {
        self.id == other.id
            && self.activity == other.activity
            && self.application == other.application
            && self.application_id == other.application_id
            && self.attachments == other.attachments
            && self.author == other.author.id
            && self.call == other.call
            && self.channel_id == other.channel_id
            && self.components == other.components
            && self.content == other.content
            && self.edited_timestamp == other.edited_timestamp
            && self.embeds == other.embeds
            && self.flags == other.flags
            && self.guild_id == other.guild_id
            && self
                .interaction
                .as_ref()
                .map_or(other.interaction.is_none(), |interaction| {
                    other
                        .interaction
                        .as_ref()
                        .is_some_and(|other_interaction| interaction == other_interaction)
                })
            && self.kind == other.kind
            && self.member == other.member
            && self.mention_channels == other.mention_channels
            && self.mention_everyone == other.mention_everyone
            && self.mention_roles == other.mention_roles
            && self.mentions.len() == other.mentions.len()
            && self
                .mentions
                .iter()
                .zip(other.mentions.iter())
                .all(|(user_id, mention)| user_id == &mention.id)
            && self.pinned == other.pinned
            && self.reactions == other.reactions
            && self.reference == other.reference
            && self.role_subscription_data == other.role_subscription_data
            && self.sticker_items == other.sticker_items
            && self.thread_id == other.thread.as_ref().map(|thread| thread.id)
            && self.timestamp == other.timestamp
            && self.tts == other.tts
            && self.webhook_id == other.webhook_id
    }
}
