use tulpje_cache::Cache;
use tulpje_framework::Error;
use tulpje_shared::color::{self, Color};

use twilight_http::{Client, error::ErrorType, response::StatusCode};
use twilight_model::{
    channel::{Channel, ChannelType, message::Component},
    guild::{Permissions, Role},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    },
};
use twilight_util::{
    builder::message::{ContainerBuilder, TextDisplayBuilder},
    permission_calculator::PermissionCalculator,
};

use crate::{context::CommandContext, responses};

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

/// check whether the specified user has the required permissions and
/// communicates to the end user if it doesn't.
///
/// returns a boolean indicating whether the permissions were present
pub(crate) async fn handle_permissions(
    ctx: &CommandContext,
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
    channel: &Channel,
    permissions: Permissions,
) -> Result<bool, Error> {
    let everyone_role = get_everyone_role(&ctx.client, &ctx.services.cache, guild_id).await?;
    let member_roles =
        get_member_roles(&ctx.client, &ctx.services.cache, user_id, guild_id).await?;
    let member_role_permissions: Vec<_> =
        member_roles.iter().map(|r| (r.id, r.permissions)).collect();

    let calculator = PermissionCalculator::new(
        guild_id,
        user_id,
        everyone_role.permissions,
        &member_role_permissions,
    );

    // calculate effective permissions
    let calculated_permissions = calculator.in_channel(
        channel.kind,
        &channel.permission_overwrites.clone().unwrap_or_default(),
    );

    // calculate missing permissions
    let missing_permissions = permissions.difference(calculated_permissions);

    // return if user has all permissions
    if missing_permissions.is_empty() {
        return Ok(true);
    }

    // get permission names
    let mut permission_names: Vec<_> = missing_permissions.iter_names().map(|(k, _)| k).collect();

    // pop the last one for string formatting
    let last_permission = permission_names
        .pop()
        .expect("missing_permissions isn't empty so shouldn't fail");

    // format the permission string
    let permissions_string = if !permission_names.is_empty() {
        format!(
            "{} and {} permissions",
            permission_names.join(", "),
            last_permission
        )
    } else {
        format!("{} permission", last_permission)
    };

    // inform the user
    responses::error(
        ctx,
        &format!("bot is missing {permissions_string} in <#{}>", channel.id),
    )
    .await?;

    Ok(false)
}

pub(crate) async fn find_channel_by_name(
    client: &Client,
    guild_id: Id<GuildMarker>,
    name: &str,
    kind: ChannelType,
) -> Result<Option<Channel>, Error> {
    Ok(client
        .guild_channels(guild_id)
        .await?
        .models()
        .await?
        .into_iter()
        .find(|c| {
            c.kind == kind
                && c.name
                    .as_ref()
                    .expect("guild channels have names")
                    .to_lowercase()
                    == name.to_lowercase()
        }))
}

pub(crate) async fn handle_channel_from_id(
    ctx: &CommandContext,
    guild_id: Id<GuildMarker>,
    channel_id: Id<ChannelMarker>,
) -> Result<Option<Channel>, Error> {
    // try and retrieve the channel, handling any errors
    match ctx.client.channel(channel_id).await {
        Ok(resp) => {
            let channel = resp.model().await?;

            // ensure channel is in current guild, otherwise send an error to user
            if channel
                .guild_id
                .is_some_and(|channel_guild_id| channel_guild_id != guild_id)
            {
                responses::channel_not_found(ctx, channel_id).await?;
                Ok(None)
            } else {
                Ok(Some(channel))
            }
        }
        Err(err) => match err.kind() {
            // NOT_FOUND indicates the channel doesn't exist, FORBIDDEN indicates the bot
            // doesn't have access to it, either way inform the user the same
            ErrorType::Response { status, .. }
                if *status == StatusCode::NOT_FOUND || *status == StatusCode::FORBIDDEN =>
            {
                responses::channel_not_found(ctx, channel_id).await?;
                Ok(None)
            }
            _ => Err(err.into()),
        },
    }
}

pub(crate) fn parse_channel_ref(channel_ref: &str) -> Option<Id<ChannelMarker>> {
    if !channel_ref.starts_with("<#") {
        return None;
    }

    channel_ref
        .trim()
        .trim_start_matches("<#")
        .trim_end_matches(">")
        .parse()
        .ok()
}
