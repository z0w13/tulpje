use twilight_model::{
    channel::{
        ChannelType,
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    },
    guild::Permissions,
    id::marker::GenericMarker,
};

use crate::{
    context::CommandContext,
    modules::pk::notify::db,
    responses,
    util::{find_channel_by_name, handle_channel_from_id, handle_permissions, parse_channel_ref},
};
use tulpje_framework::Error;

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let channel_name_or_ref = ctx.get_arg_string("channel")?;
    let bot_user = ctx.client.current_user().await?.model().await?;
    let required_permissions = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES;

    let existing_channel = if let Some(channel_id) = parse_channel_ref(&channel_name_or_ref) {
        // handle channel references
        let Some(channel) = handle_channel_from_id(&ctx, guild.id, channel_id).await? else {
            return Ok(());
        };
        Some(channel)
    } else {
        // handle channel names
        find_channel_by_name(
            &ctx.client,
            guild.id,
            &channel_name_or_ref,
            ChannelType::GuildText,
        )
        .await?
    };

    let channel = if let Some(channel) = existing_channel {
        // if existing channel, check permissions and return if missing
        if !handle_permissions(&ctx, guild.id, bot_user.id, &channel, required_permissions).await? {
            return Ok(());
        }
        channel
    } else {
        // otherwise create the channel
        let permission_overwrites = vec![PermissionOverwrite {
            allow: required_permissions,
            deny: Permissions::empty(),
            id: bot_user.id.cast::<GenericMarker>(),
            kind: PermissionOverwriteType::Member,
        }];
        ctx.client
            .create_guild_channel(guild.id, &channel_name_or_ref)
            .permission_overwrites(&permission_overwrites)
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
