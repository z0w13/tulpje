use tulpje_framework::Error;
use twilight_model::{
    channel::{
        ChannelType,
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    },
    guild::Permissions,
    id::marker::GenericMarker,
};

use super::db;
use crate::{
    context::CommandContext,
    modules::pk::{
        db::{get_guild_settings_for_id, get_system},
        fronters::shared::handle_private_front,
        util::SystemRef,
    },
    responses,
    util::{find_channel_by_name, handle_channel_from_id, handle_permissions, parse_channel_ref},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;

    let Some(guild_settings) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        responses::error(&ctx, "PluralKit module not set-up, please run `/pk setup`").await?;
        return Ok(());
    };

    let system_ref = SystemRef::Id(guild_settings.system_id);
    let system = get_system(&ctx.services.db, &system_ref).await?;
    let display_name = system.map_or_else(|| system_ref.to_string(), |s| s.name.unwrap_or(s.id));

    // inform the user if their front is private
    if handle_private_front(
        &ctx,
        system_ref.clone(),
        &format!("Front for system `{display_name}` is private, please set it to public to use the fronter list")
    )
    .await?
    {
        return Ok(());
    }

    // create or get the front category
    let category_name_or_ref = ctx.get_arg_string("name")?;
    let bot_user = ctx.client.current_user().await?.model().await?;
    let required_permissions = Permissions::MANAGE_CHANNELS | Permissions::VIEW_CHANNEL;

    let existing_category = if let Some(channel_id) = parse_channel_ref(&category_name_or_ref) {
        // handle channel references
        let Some(channel) = handle_channel_from_id(&ctx, guild.id, channel_id).await? else {
            return Ok(());
        };
        Some(channel)
    } else {
        // handle channel names
        find_channel_by_name(&ctx.client, guild.id, &category_name_or_ref).await?
    };

    let fronters_category = if let Some(category) = existing_category {
        // if existing channel, check permissions and return if missing
        if !handle_permissions(&ctx, guild.id, bot_user.id, &category, required_permissions).await?
        {
            return Ok(());
        }
        category
    } else {
        // otherwise create the channel
        let permission_overwrites = vec![
            PermissionOverwrite {
                deny: Permissions::VIEW_CHANNEL,
                allow: Permissions::empty(),
                id: guild.id.cast(),
                kind: PermissionOverwriteType::Role,
            },
            PermissionOverwrite {
                allow: required_permissions,
                deny: Permissions::empty(),
                id: bot_user.id.cast::<GenericMarker>(),
                kind: PermissionOverwriteType::Member,
            },
        ];
        ctx.client
            .create_guild_channel(guild.id, &category_name_or_ref)
            .permission_overwrites(&permission_overwrites)
            .kind(ChannelType::GuildCategory)
            .await?
            .model()
            .await?
    };

    // Save category into db
    db::save_fronter_category(&ctx.services.db, guild.id, fronters_category.id).await?;

    // Inform user of success
    responses::success(&ctx, "Fronter category succesfully set-up!").await?;
    Ok(())
}
