use twilight_http::{Client, error::ErrorType, response::StatusCode};
use twilight_model::{
    channel::{Channel, ChannelType},
    guild::Permissions,
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker},
    },
};

use crate::{
    context::CommandContext, modules::pk::notify::db, responses, util::handle_permissions,
};
use tulpje_framework::Error;

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let channel_name = ctx.get_arg_string("channel")?;
    let bot_user = ctx.client.current_user().await?.model().await?;
    let required_permissions = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES;

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
                        responses::channel_not_found(&ctx, channel_id).await?;
                        return Ok(());
                    }
                    _ => return Err(err.into()),
                };
            }
        };

        // We need to separately handle channels the bot _can_ access but that are
        // outside of the user's guild. Give the same error message though
        if channel.guild_id.is_some_and(|i| i != guild.id) {
            responses::channel_not_found(&ctx, channel.id).await?;
            return Ok(());
        }

        // check permissions
        if !handle_permissions(&ctx, guild.id, bot_user.id, &channel, required_permissions).await? {
            return Ok(());
        }

        channel
    } else if let Some(channel) = find_channel_by_name(&ctx.client, guild.id, &channel_name).await?
        && handle_permissions(&ctx, guild.id, bot_user.id, &channel, required_permissions).await?
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
    responses::success(
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
