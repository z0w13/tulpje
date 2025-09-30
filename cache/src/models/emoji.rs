use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::Emoji,
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker, RoleMarker, UserMarker},
    },
};

use crate::{Cache, Error, GuildResource};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachedEmoji {
    pub animated: bool,
    pub available: bool,
    pub id: Id<EmojiMarker>,
    pub managed: bool,
    pub name: String,
    pub require_colons: bool,
    pub roles: Vec<Id<RoleMarker>>,
    pub user_id: Option<Id<UserMarker>>,
}

impl From<Emoji> for CachedEmoji {
    fn from(emoji: Emoji) -> Self {
        let Emoji {
            animated,
            available,
            id,
            managed,
            name,
            require_colons,
            roles,
            user,
        } = emoji;

        Self {
            animated,
            available,
            id,
            managed,
            name,
            require_colons,
            roles,
            user_id: user.map(|user| user.id),
        }
    }
}

impl PartialEq<Emoji> for CachedEmoji {
    fn eq(&self, other: &Emoji) -> bool {
        self.id == other.id
            && self.animated == other.animated
            && self.managed == other.managed
            && self.name == other.name
            && self.require_colons == other.require_colons
            && self.roles == other.roles
            && self.user_id == other.user.as_ref().map(|user| user.id)
            && self.available == other.available
    }
}

impl Cache {
    pub(crate) async fn cache_emojis(
        &self,
        guild_id: Id<GuildMarker>,
        emojis: Vec<Emoji>,
    ) -> Result<(), Error> {
        let guild_emojis = self.guild_emojis.members(&guild_id).await?;
        if !guild_emojis.is_empty() {
            let incoming: Vec<_> = emojis.iter().map(|e| e.id).collect();

            let removal_filter: Vec<_> = guild_emojis
                .iter()
                .copied()
                .filter(|e| !incoming.contains(e))
                .collect();

            self.guild_emojis
                .remove_multi(&guild_id, &removal_filter)
                .await?;

            self.emojis.remove_multi(&removal_filter).await?;
        }

        for emoji in &emojis {
            self.cache_emoji(guild_id, emoji).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_emoji(
        &self,
        guild_id: Id<GuildMarker>,
        emoji: &Emoji,
    ) -> Result<(), Error> {
        if let Some(user) = emoji.user.as_ref() {
            self.cache_user(user, Some(guild_id)).await?;
        }

        if let Some(cached_emoji) = self.emojis.get(&emoji.id).await? {
            if cached_emoji.value == *emoji {
                return Ok(());
            }
        }

        let emoji_id = emoji.id;
        let cached = CachedEmoji::from(emoji.clone());

        self.emojis
            .insert(
                &emoji_id,
                &GuildResource {
                    guild_id,
                    value: cached,
                },
            )
            .await?;

        self.guild_emojis.insert(&guild_id, &emoji_id).await?;

        Ok(())
    }
}
