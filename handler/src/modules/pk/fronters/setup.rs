use pkrs_fork::{client::PkClient, model::PkId};
use reqwest::StatusCode;
use tulpje_framework::Error;

use super::{db, shared::create_or_get_fronter_channel};
use crate::{
    context::CommandContext,
    modules::pk::{
        db::get_guild_settings_for_id, fronters::shared::handle_private_front, util::SystemRef,
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

    // inform the user if their front is private
    if handle_private_front(
        &ctx,
        &PkClient::default(),
        SystemRef::Id(guild_settings.system_id.clone()),
        &format!(
            "Front for system `{}` is private, please set it to public to use the fronter list",
            guild_settings.system_id
        ),
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
