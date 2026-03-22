use tulpje_framework::Error;

use super::{db, shared::create_or_get_fronter_channel};
use crate::{
    context::CommandContext,
    modules::pk::db::get_guild_settings_for_id,
    util::{error_response, success_response},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;

    if get_guild_settings_for_id(&ctx.services.db, guild.id)
        .await?
        .is_none()
    {
        error_response(&ctx, "PluralKit module not set-up, please run `/pk setup`").await?;
        return Ok(());
    };

    let name = ctx.get_arg_string("name")?;
    let fronters_category = create_or_get_fronter_channel(&ctx.client, &guild, name).await?;

    // Save category into db
    db::save_fronter_category(&ctx.services.db, guild.id, fronters_category.id).await?;

    // Inform user of success
    success_response(&ctx, "Fronter category succesfully set-up!").await?;
    Ok(())
}
