use std::{fmt::Display, str::FromStr};

use pkrs_fork::model::Member;
use tulpje_framework::{Error, color};
use twilight_model::id::{Id, marker::UserMarker};
use uuid::Uuid;

use crate::{context::CommandContext, responses};

pub(crate) fn get_member_name(member: &Member) -> String {
    member
        .display_name
        .clone()
        .unwrap_or_else(|| member.name.clone())
}

pub(crate) fn normalize_short_id(short_id: &str) -> String {
    short_id.trim().replace("-", "").to_ascii_lowercase()
}

#[derive(Debug, Clone)]
pub(crate) enum SystemRef {
    DiscordId(Id<UserMarker>),
    Uuid(Uuid),
    Id(String),
}

impl FromStr for SystemRef {
    type Err = tulpje_framework::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // PluralKit short IDs
        if s.len() >= 5 && s.len() <= 7 {
            let normalized = normalize_short_id(s);
            if normalized.len() >= 5 && normalized.len() <= 6 {
                return Ok(Self::Id(normalized));
            }
        }

        // Discord Snowflakes
        if let Ok(discord_id) = Id::<UserMarker>::from_str(s) {
            return Ok(Self::DiscordId(discord_id));
        }

        // UUIDs
        if let Ok(uuid) = Uuid::from_str(s) {
            return Ok(Self::Uuid(uuid));
        }

        Err(format!("Couldn't parse '{s}' into SystemRef").into())
    }
}

impl From<SystemRef> for String {
    fn from(value: SystemRef) -> Self {
        match value {
            SystemRef::Id(id) => id,
            SystemRef::DiscordId(user_id) => user_id.to_string(),
            SystemRef::Uuid(uuid) => uuid.to_string(),
        }
    }
}

impl Display for SystemRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => id.fmt(f),
            Self::DiscordId(user_id) => user_id.fmt(f),
            Self::Uuid(uuid) => uuid.fmt(f),
        }
    }
}

/// try to parse a system ref, and let the end user know if it fails
/// returns None if failed to parse
pub(crate) async fn handle_system_ref(
    ctx: &CommandContext,
    system_ref: &str,
) -> Result<Option<SystemRef>, Error> {
    match system_ref.parse() {
        Ok(system_ref) => Ok(Some(system_ref)),
        Err(_) => {
            responses::error(
                ctx,
                &format!(
                    "Invalid system reference `{system_ref}`, are you sure you entered it correctly?",
                ),
            )
            .await?;
            Ok(None)
        }
    }
}

pub(crate) fn pk_color_to_discord(hex: Option<String>) -> u32 {
    hex.map_or(color::roles::DEFAULT, |hex| {
        color::Color::from_str(&hex).unwrap_or(color::roles::DEFAULT)
    })
    .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pk_color_to_discord() {
        assert_eq!(
            pk_color_to_discord(Some("unparseable".to_string())),
            color::roles::DEFAULT.0
        );
        assert_eq!(pk_color_to_discord(None), color::roles::DEFAULT.0);
    }
}
