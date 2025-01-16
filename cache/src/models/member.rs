use std::ops::Deref;

use serde::{Deserialize, Serialize};
use twilight_model::{
    application::interaction::InteractionMember,
    gateway::payload::incoming::MemberUpdate,
    guild::{Member, MemberFlags, PartialMember},
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};

use crate::{Cache, Error};

/// Computed components required to complete a full cached interaction member
/// by implementing [`CacheableMember`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComputedInteractionMember {
    pub avatar: Option<ImageHash>,
    pub deaf: Option<bool>,
    pub interaction_member: InteractionMember,
    pub mute: Option<bool>,
    pub user_id: Id<UserMarker>,
}

impl Deref for ComputedInteractionMember {
    type Target = InteractionMember;

    fn deref(&self) -> &Self::Target {
        &self.interaction_member
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachedMember {
    pub avatar: Option<ImageHash>,
    pub communication_disabled_until: Option<Timestamp>,
    pub deaf: Option<bool>,
    pub flags: MemberFlags,
    pub joined_at: Option<Timestamp>,
    pub mute: Option<bool>,
    pub nick: Option<String>,
    pub pending: bool,
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user_id: Id<UserMarker>,
}

impl CachedMember {
    pub(crate) fn update_with_member_update(&mut self, member_update: &MemberUpdate) {
        self.avatar = member_update.avatar;
        self.deaf = member_update.deaf.or(self.deaf);
        self.mute = member_update.mute.or(self.mute);
        self.nick.clone_from(&member_update.nick);
        self.roles.clone_from(&member_update.roles);
        self.joined_at = member_update.joined_at;
        self.pending = member_update.pending;
        self.communication_disabled_until = member_update.communication_disabled_until;
    }
}

impl From<Member> for CachedMember {
    fn from(member: Member) -> Self {
        let Member {
            avatar,
            communication_disabled_until,
            deaf,
            flags,
            joined_at,
            mute,
            nick,
            pending,
            premium_since,
            roles,
            user,
        } = member;

        Self {
            avatar,
            communication_disabled_until,
            deaf: Some(deaf),
            flags,
            joined_at,
            mute: Some(mute),
            nick,
            pending,
            premium_since,
            roles,
            user_id: user.id,
        }
    }
}

impl From<(Id<UserMarker>, PartialMember)> for CachedMember {
    fn from((user_id, member): (Id<UserMarker>, PartialMember)) -> Self {
        #[expect(
            clippy::unneeded_field_pattern,
            reason = "clearer that we're explicitly skipping those fields"
        )]
        let PartialMember {
            avatar,
            communication_disabled_until,
            deaf,
            flags,
            joined_at,
            mute,
            nick,
            permissions: _,
            premium_since,
            roles,
            user,
        } = member;

        Self {
            avatar,
            communication_disabled_until,
            deaf: Some(deaf),
            flags,
            joined_at,
            mute: Some(mute),
            nick,
            pending: false,
            premium_since,
            roles,
            user_id: user.map_or(user_id, |user| user.id),
        }
    }
}

impl From<ComputedInteractionMember> for CachedMember {
    fn from(member: ComputedInteractionMember) -> Self {
        let ComputedInteractionMember {
            avatar,
            deaf,
            mute,
            user_id,
            interaction_member,
        } = member;

        #[expect(
            clippy::unneeded_field_pattern,
            reason = "clearer that we're explicitly skipping those fields"
        )]
        let InteractionMember {
            avatar: _,
            communication_disabled_until,
            flags,
            joined_at,
            nick,
            pending,
            permissions: _,
            premium_since,
            roles,
        } = interaction_member;

        Self {
            avatar,
            communication_disabled_until,
            deaf,
            flags,
            joined_at,
            mute,
            nick,
            pending,
            premium_since,
            roles,
            user_id,
        }
    }
}

impl PartialEq<Member> for CachedMember {
    fn eq(&self, other: &Member) -> bool {
        self.avatar == other.avatar
            && self.communication_disabled_until == other.communication_disabled_until
            && self.deaf == Some(other.deaf)
            && self.joined_at == other.joined_at
            && self.mute == Some(other.mute)
            && self.nick == other.nick
            && self.pending == other.pending
            && self.premium_since == other.premium_since
            && self.roles == other.roles
            && self.user_id == other.user.id
    }
}

impl PartialEq<PartialMember> for CachedMember {
    fn eq(&self, other: &PartialMember) -> bool {
        self.communication_disabled_until == other.communication_disabled_until
            && self.deaf == Some(other.deaf)
            && self.joined_at == other.joined_at
            && self.mute == Some(other.mute)
            && self.nick == other.nick
            && self.premium_since == other.premium_since
            && self.roles == other.roles
    }
}

impl PartialEq<InteractionMember> for CachedMember {
    fn eq(&self, other: &InteractionMember) -> bool {
        self.joined_at == other.joined_at
            && self.nick == other.nick
            && self.premium_since == other.premium_since
            && self.roles == other.roles
    }
}

impl Cache {
    pub(crate) async fn cache_members(
        &self,
        guild_id: Id<GuildMarker>,
        members: impl IntoIterator<Item = Member>,
    ) -> Result<(), Error> {
        for member in members {
            self.cache_member(guild_id, member).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_member(
        &self,
        guild_id: Id<GuildMarker>,
        member: Member,
    ) -> Result<(), Error> {
        let member_id = member.user.id;
        let id = (guild_id, member_id);

        if self.members.get(&id).await?.is_some_and(|m| m == member) {
            return Ok(());
        }

        self.cache_user(&member.user, Some(guild_id)).await?;
        self.members
            .insert(&id, &CachedMember::from(member.clone()))
            .await?;
        self.guild_members.insert(&guild_id, &member_id).await?;

        Ok(())
    }

    pub(crate) async fn cache_borrowed_partial_member(
        &self,
        guild_id: Id<GuildMarker>,
        member: &PartialMember,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error> {
        let id = (guild_id, user_id);

        if self.members.get(&id).await?.is_some_and(|m| m == *member) {
            return Ok(());
        }

        self.guild_members.insert(&guild_id, &user_id).await?;

        self.members
            .insert(&id, &CachedMember::from((user_id, member.clone())))
            .await?;

        Ok(())
    }

    pub(crate) async fn cache_borrowed_interaction_member(
        &self,
        guild_id: Id<GuildMarker>,
        member: &InteractionMember,
        user_id: Id<UserMarker>,
    ) -> Result<(), Error> {
        let id = (guild_id, user_id);

        let (avatar, deaf, mute) = match self.members.get(&id).await? {
            Some(m) if &m == member => return Ok(()),
            Some(m) => (m.avatar, m.deaf, m.mute),
            None => (None, None, None),
        };

        self.guild_members.insert(&guild_id, &user_id).await?;

        let cached = CachedMember::from(ComputedInteractionMember {
            avatar,
            deaf,
            interaction_member: member.clone(),
            mute,
            user_id,
        });

        self.members.insert(&id, &cached).await?;

        Ok(())
    }
}
