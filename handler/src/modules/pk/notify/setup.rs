use twilight_http::Client;
use twilight_model::{
    channel::{Channel, ChannelType},
    guild::{Permissions, Role},
    id::{
        Id,
        marker::{GuildMarker, RoleMarker, UserMarker},
    },
};
use twilight_util::permission_calculator::PermissionCalculator;

use tulpje_cache::Cache;
use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::notify::db,
    util::{error_response, success_response},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let channel_name = ctx.get_arg_string("channel")?;

    // get the channel if it already exists

    let channel = if let Some(channel) =
        find_channel_by_name(&ctx.client, guild.id, &channel_name).await?
        && handle_channel_permissions(&ctx, guild.id, &channel).await?
    {
        channel
    } else {
        ctx.client
            .create_guild_channel(guild.id, &channel_name)
            .kind(ChannelType::GuildText)
            .await?
            .model()
            .await?
    };

    db::save_notify_channel(&ctx.services.db, guild.id, channel.id).await?;
    success_response(
        &ctx,
        &format!("bot will notify you of front changes in <#{}>", channel.id),
    )
    .await?;

    Ok(())
}

async fn find_channel_by_name(
    client: &Client,
    guild_id: Id<GuildMarker>,
    name: &str,
) -> Result<Option<Channel>, Error> {
    Ok(client
        .guild_channels(guild_id)
        .await?
        .models()
        .await?
        .into_iter()
        .find(|c| {
            c.name
                .as_ref()
                .expect("guild channels have names")
                .to_lowercase()
                == name.to_lowercase()
        }))
}

/// check channel permissions and inform user of missing ones
async fn handle_channel_permissions(
    ctx: &CommandContext,
    guild_id: Id<GuildMarker>,
    channel: &Channel,
) -> Result<bool, Error> {
    let current_user = ctx.client.current_user().await?.model().await?;
    let everyone_role = get_everyone_role(&ctx.client, &ctx.services.cache, guild_id).await?;
    let member_roles =
        get_member_roles(&ctx.client, &ctx.services.cache, current_user.id, guild_id).await?;
    let member_role_permissions: Vec<_> =
        member_roles.iter().map(|r| (r.id, r.permissions)).collect();

    let calculator = PermissionCalculator::new(
        guild_id,
        current_user.id,
        everyone_role.permissions,
        &member_role_permissions,
    );

    let calculated_permissions = calculator.in_channel(
        ChannelType::GuildText,
        &channel.permission_overwrites.clone().unwrap_or_default(),
    );

    // NOTE: We need to check VIEW_CHANNEL too, because of implicit permissions
    //       see: https://docs.discord.com/developers/topics/permissions#implicit-permissions
    if !calculated_permissions.contains(Permissions::VIEW_CHANNEL) {
        error_response(
            ctx,
            &format!(
                "bot is missing VIEW_CHANNEL permission in <#{}>",
                channel.id
            ),
        )
        .await?;
        return Ok(false);
    }

    if !calculated_permissions.contains(Permissions::SEND_MESSAGES) {
        error_response(
            ctx,
            &format!(
                "bot is missing SEND_MESSAGES permission in <#{}>",
                channel.id
            ),
        )
        .await?;
        return Ok(false);
    }

    Ok(true)
}

async fn get_member_roles(
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

async fn get_everyone_role(
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
