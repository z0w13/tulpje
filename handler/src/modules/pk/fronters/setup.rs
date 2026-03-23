use tulpje_framework::Error;
use twilight_http::Client;
use twilight_model::{
    channel::{
        Channel, ChannelType,
        permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    },
    guild::{Guild, Permissions},
    id::marker::GenericMarker,
};

use super::db;
use crate::{
    context::CommandContext,
    modules::pk::{
        db::{get_guild_settings_for_id, get_system},
        fronters::shared::{get_fronter_category, handle_private_front},
        util::SystemRef,
    },
    util::{error_response, success_response},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;

    let Some(guild_settings) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        error_response(&ctx, "PluralKit module not set-up, please run `/pk setup`").await?;
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
    let name = ctx.get_arg_string("name")?;
    let fronters_category = create_or_get_fronter_channel(&ctx.client, &guild, name).await?;

    // Save category into db
    db::save_fronter_category(&ctx.services.db, guild.id, fronters_category.id).await?;

    // Inform user of success
    success_response(&ctx, "Fronter category succesfully set-up!").await?;
    Ok(())
}

async fn create_or_get_fronter_channel(
    client: &Client,
    guild: &Guild,
    cat_name: String,
) -> Result<Channel, Error> {
    if let Some(fronters_category) =
        get_fronter_category(client, guild, Some(cat_name.clone())).await?
    {
        return Ok(fronters_category);
    }

    // get the bot's user id
    let user_id = client.current_user().await?.model().await?.id;

    // define permissions
    let permissions = vec![
        PermissionOverwrite {
            deny: Permissions::VIEW_CHANNEL,
            allow: Permissions::empty(),
            id: guild.id.cast(),
            kind: PermissionOverwriteType::Role,
        },
        PermissionOverwrite {
            allow: Permissions::MANAGE_CHANNELS | Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
            id: user_id.cast::<GenericMarker>(),
            kind: PermissionOverwriteType::Member,
        },
    ];

    Ok(client
        .create_guild_channel(guild.id, &cat_name)
        .permission_overwrites(&permissions)
        .kind(ChannelType::GuildCategory)
        .await?
        .model()
        .await?)
}
