use std::str::FromStr;

use pkrs_fork::model::Member;
use tulpje_shared::color;
use twilight_model::id::{Id, marker::UserMarker};
use uuid::Uuid;

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
