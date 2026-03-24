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

    let category_title = ctx.get_arg_string("title")?;
    let bot_user = ctx.client.current_user().await?.model().await?;
    let required_permissions = Permissions::MANAGE_CHANNELS | Permissions::VIEW_CHANNEL;

    // define required permissions
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

    // create the category
    let fronters_category = ctx
        .client
        .create_guild_channel(guild.id, &category_title)
        .permission_overwrites(&permission_overwrites)
        .kind(ChannelType::GuildCategory)
        .await?
        .model()
        .await?;

    // Save category into db
    db::save_fronter_category(&ctx.services.db, guild.id, fronters_category.id).await?;

    // Inform user of success
    responses::success(&ctx, "Fronter category succesfully set-up!").await?;
    Ok(())
}
