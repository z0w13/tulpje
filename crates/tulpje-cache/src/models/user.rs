use twilight_model::{
    id::{Id, marker::GuildMarker},
    user::CurrentUser,
};

use crate::{Cache, Error};

pub use twilight_model::user::CurrentUser as CachedCurrentUser;
pub use twilight_model::user::User as CachedUser;

impl Cache {
    pub(crate) async fn cache_user(
        &self,
        user: &CachedUser,
        guild_id: Option<Id<GuildMarker>>,
    ) -> Result<(), Error> {
        if let Some(cached_user) = self.users.get(&user.id).await?
            && cached_user == *user
        {
            if let Some(guild_id) = guild_id {
                self.user_guilds.insert(&user.id, &guild_id).await?;
            }

            return Ok(());
        }

        self.users.insert(&user.id, user).await?;
        if let Some(guild_id) = guild_id {
            self.user_guilds.insert(&user.id, &guild_id).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_current_user(&self, user: &CurrentUser) -> Result<(), Error> {
        self.current_user.set(user.clone()).await?;
        Ok(())
    }
}
