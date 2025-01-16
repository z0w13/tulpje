use twilight_model::{
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

use crate::{Cache, Error, GuildResource};

pub use twilight_model::guild::Role as CachedRole;

impl Cache {
    pub(crate) async fn cache_roles(
        &self,
        guild_id: Id<GuildMarker>,
        roles: impl IntoIterator<Item = Role>,
    ) -> Result<(), Error> {
        for role in roles {
            self.cache_role(guild_id, role).await?;
        }

        Ok(())
    }

    pub(crate) async fn cache_role(
        &self,
        guild_id: Id<GuildMarker>,
        role: Role,
    ) -> Result<(), Error> {
        self.guild_roles.insert(&guild_id, &role.id).await?;
        self.roles
            .insert(
                &role.id.clone(),
                &GuildResource {
                    guild_id,
                    value: role,
                },
            )
            .await?;

        Ok(())
    }

    pub(crate) async fn delete_role(
        &self,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<(), Error> {
        self.roles.remove(&role_id).await?;
        self.guild_roles.remove(&guild_id, &role_id).await?;

        Ok(())
    }
}
