use twilight_http::{Client, error::ErrorType, response::StatusCode};
use twilight_model::{
    channel::{Channel, ChannelType},
    guild::{Permissions, Role},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    },
};
use twilight_util::permission_calculator::PermissionCalculator;

use tulpje_cache::Cache;
use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::notify::db,
    util::{error_response, get_everyone_role, get_member_roles, success_response},
};

async fn channel_not_found_response(
    ctx: &CommandContext,
    id: Id<ChannelMarker>,
) -> Result<(), Error> {
    error_response(
        ctx,
        &format!(
            "### Error\nCouldn't find channel, are you sure it's in this server and the bot can access it?\n\nChannel ID: `{id}`",
        ),
    )
    .await?;

    Ok(())
}

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let channel_name = ctx.get_arg_string("channel")?;

    let channel = if channel_name.starts_with("<#") {
        // handle channel references

        // parse the channel id
        let channel_id: Id<ChannelMarker> = channel_name
            .trim()
            .trim_start_matches("<#")
            .trim_end_matches(">")
            .parse()?;

        // try and retrieve the channel, handling any errors
        let channel = match ctx.client.channel(channel_id).await {
            Ok(resp) => resp.model().await?,
            Err(err) => {
                match err.kind() {
                    // NOT_FOUND indicates the channel doesn't exist, FORBIDDEN indicates the bot
                    // doesn't have access to it, either way inform the user the same
                    ErrorType::Response { status, .. }
                        if *status == StatusCode::NOT_FOUND || *status == StatusCode::FORBIDDEN =>
                    {
                        channel_not_found_response(&ctx, channel_id).await?;
                        return Ok(());
                    }
                    _ => return Err(err.into()),
                };
            }
        };

        // We need to separately handle channels the bot _can_ access but that are
        // outside of the user's guild. Give the same error message though
        if channel.guild_id.is_some_and(|i| i != guild.id) {
            channel_not_found_response(&ctx, channel.id).await?;
            return Ok(());
        }

        // check permissions
        if !handle_channel_permissions(&ctx, guild.id, &channel).await? {
            return Ok(());
        }

        channel
    } else if let Some(channel) = find_channel_by_name(&ctx.client, guild.id, &channel_name).await?
        && handle_channel_permissions(&ctx, guild.id, &channel).await?
    {
        // return the channel if we found it by name
        channel
    } else {
        // if all else fails make a new one
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
