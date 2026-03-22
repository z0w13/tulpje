use tulpje_framework::Error;

use super::{db, shared::update_fronter_channels};
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

    let Some(gs) = get_guild_settings_for_id(&ctx.services.db, guild.id).await? else {
        error_response(&ctx, "PluralKit module not set-up, please run `/pk setup`").await?;
        return Ok(());
    };

    let Some(cat_id) = db::get_fronter_category(&ctx.services.db, guild.id).await? else {
        error_response(
            &ctx,
            "Fronter category not set-up, please run `/pk fronters setup`",
        )
        .await?;
        return Ok(());
    };

    let cat = ctx.client().channel(*cat_id).await?.model().await?;

    cat.guild_id
        .ok_or_else(|| format!("channel {} isn't a guild channel", cat_id))?;

    update_fronter_channels(&ctx.client(), &ctx.services.cache, guild, &gs, cat, None).await?;

    success_response(&ctx, "Fronter category updated!").await?;
    Ok(())
}
