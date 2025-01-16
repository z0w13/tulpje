use serde::{Deserialize, Serialize};
use twilight_model::{
    gateway::presence::{Activity, ClientStatus, Presence, Status},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

use crate::{Cache, Error};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CachedPresence {
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
    pub guild_id: Id<GuildMarker>,
    pub status: Status,
    pub user_id: Id<UserMarker>,
}

impl From<Presence> for CachedPresence {
    fn from(presence: Presence) -> Self {
        let Presence {
            activities,
            client_status,
            guild_id,
            status,
            user,
        } = presence;

        Self {
            activities,
            client_status,
            guild_id,
            status,
            user_id: user.id(),
        }
    }
}

impl PartialEq<Presence> for CachedPresence {
    fn eq(&self, other: &Presence) -> bool {
        self.activities == other.activities
            && self.client_status == other.client_status
            && self.guild_id == other.guild_id
            && self.status == other.status
            && self.user_id == other.user.id()
    }
}

impl Cache {
    pub(crate) async fn cache_presences(
        &self,
        guild_id: Id<GuildMarker>,
        presences: impl IntoIterator<Item = Presence>,
    ) -> Result<(), Error> {
        for presence in presences {
            self.cache_presence(guild_id, presence).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_presence(
        &self,
        guild_id: Id<GuildMarker>,
        presence: Presence,
    ) -> Result<(), Error> {
        self.guild_presences
            .insert(&guild_id, &presence.user.id())
            .await?;

        self.presences
            .insert(
                &(guild_id, presence.user.id()),
                &CachedPresence::from(presence),
            )
            .await?;

        Ok(())
    }
}
