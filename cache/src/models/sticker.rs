use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::message::{
        Sticker,
        sticker::{StickerFormatType, StickerType},
    },
    id::{
        Id,
        marker::{GuildMarker, StickerMarker, StickerPackMarker, UserMarker},
    },
};

use crate::{Cache, Error, GuildResource};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachedSticker {
    available: bool,
    description: String,
    format_type: StickerFormatType,
    guild_id: Option<Id<GuildMarker>>,
    id: Id<StickerMarker>,
    kind: StickerType,
    name: String,
    pack_id: Option<Id<StickerPackMarker>>,
    sort_value: Option<u64>,
    tags: String,
    user_id: Option<Id<UserMarker>>,
}

impl From<Sticker> for CachedSticker {
    fn from(sticker: Sticker) -> Self {
        let Sticker {
            available,
            description,
            format_type,
            guild_id,
            id,
            kind,
            name,
            pack_id,
            sort_value,
            tags,
            user,
        } = sticker;

        Self {
            available,
            description: description.unwrap_or_default(),
            format_type,
            guild_id,
            id,
            kind,
            name,
            pack_id,
            sort_value,
            tags,
            user_id: user.map(|user| user.id),
        }
    }
}

impl PartialEq<Sticker> for CachedSticker {
    fn eq(&self, other: &Sticker) -> bool {
        self.available == other.available
            && self.description.as_str() == other.description.as_ref().map_or("", String::as_str)
            && self.format_type == other.format_type
            && self.guild_id == other.guild_id
            && self.id == other.id
            && self.kind == other.kind
            && self.name == other.name
            && self.pack_id == other.pack_id
            && self.sort_value == other.sort_value
            && self.tags == other.tags
            && self.user_id == other.user.as_ref().map(|user| user.id)
    }
}

impl Cache {
    pub(crate) async fn cache_stickers(
        &self,
        guild_id: Id<GuildMarker>,
        stickers: Vec<Sticker>,
    ) -> Result<(), Error> {
        let guild_stickers = self.guild_stickers.members(&guild_id).await?;
        if !guild_stickers.is_empty() {
            let incoming: HashSet<_> = stickers.iter().map(|sticker| sticker.id).collect();

            let removal_filter: Vec<_> = guild_stickers
                .iter()
                .copied()
                .filter(|sticker| !incoming.contains(sticker))
                .collect();

            self.guild_stickers
                .remove_multi(&guild_id, &removal_filter)
                .await?;

            self.stickers.remove_multi(&removal_filter).await?;
        }

        for sticker in stickers {
            self.cache_sticker(guild_id, sticker).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_sticker(
        &self,
        guild_id: Id<GuildMarker>,
        sticker: Sticker,
    ) -> Result<(), Error> {
        if let Some(cached_sticker) = self.stickers.get(&sticker.id).await? {
            if cached_sticker.value == sticker {
                return Ok(());
            }
        }

        if let Some(user) = &sticker.user {
            self.cache_user(user, Some(guild_id)).await?;
        }

        self.guild_stickers.insert(&guild_id, &sticker.id).await?;
        self.stickers
            .insert(
                &sticker.id.clone(),
                &GuildResource {
                    guild_id,
                    value: sticker.into(),
                },
            )
            .await?;

        Ok(())
    }
}
