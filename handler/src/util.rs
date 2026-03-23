use tulpje_cache::Cache;
use tulpje_framework::Error;
use tulpje_shared::color::{self, Color};

use twilight_http::Client;
use twilight_model::{
    channel::message::Component,
    guild::Role,
    id::{
        Id,
        marker::{GuildMarker, RoleMarker, UserMarker},
    },
};
use twilight_util::builder::message::{ContainerBuilder, TextDisplayBuilder};

pub(crate) fn message(color: &Color, text: &str) -> Component {
    ContainerBuilder::new()
        .accent_color(Some(color.0))
        .component(TextDisplayBuilder::new(text).build())
        .build()
        .into()
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn success_message(text: &str) -> Component {
    message(&color::roles::GREEN, text)
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn error_message(text: &str) -> Component {
    message(&color::roles::RED, text)
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn warning_message(text: &str) -> Component {
    message(&color::roles::ORANGE, text)
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn info_message(text: &str) -> Component {
    message(&color::roles::BLUE, text)
}

pub(crate) async fn get_everyone_role(
    client: &Client,
    cache: &Cache,
    guild_id: Id<GuildMarker>,
) -> Result<Role, Error> {
    let role_id = guild_id.cast::<RoleMarker>();
    let everyone_role = cache.roles.get(&role_id).await?;
    if let Some(role) = everyone_role {
        return Ok(role.inner());
    }

    Ok(client.role(guild_id, role_id).await?.model().await?)
}

pub(crate) async fn get_member_roles(
    client: &Client,
    cache: &Cache,
    user_id: Id<UserMarker>,
    guild_id: Id<GuildMarker>,
) -> Result<Vec<Role>, Error> {
    let role_ids = if let Some(member) = cache.members.get(&(guild_id, user_id)).await? {
        member.roles
    } else {
        client
            .guild_member(guild_id, user_id)
            .await?
            .model()
            .await?
            .roles
    };

    let mut roles = Vec::new();
    for role_id in role_ids {
        if let Some(role) = cache.roles.get(&role_id).await? {
            roles.push(role.inner());
        } else {
            roles.push(client.role(guild_id, role_id).await?.model().await?);
        }
    }

    Ok(roles)
}
