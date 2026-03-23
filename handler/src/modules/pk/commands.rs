use pkrs_fork::model::PkId;
use reqwest::StatusCode;
use tulpje_framework::Error;

use super::db;
use crate::{context::CommandContext, modules::pk::util::handle_system_ref, responses};

// TODO: command to see current settings
pub async fn setup_pk(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;
    let user_id = ctx.event.author_id().ok_or("no author?")?;

    let Some(system_ref) = handle_system_ref(&ctx, &ctx.get_arg_string("system_id")?).await? else {
        return Ok(());
    };

    let system = match ctx
        .services
        .pk
        .get_system(&PkId(system_ref.clone().into()))
        .await
    {
        Ok(system) => system,
        Err(err)
            if err
                .status()
                .is_some_and(|status| status == StatusCode::NOT_FOUND) =>
        {
            responses::error(
                &ctx,
                &format!("### Error\nCouldn't find system `{system_ref}`"),
            )
            .await?;

            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    db::save_guild_settings(&ctx.services.db, guild.id, user_id, &system.id.0).await?;

    // Inform user of success
    responses::success(
        &ctx,
        &format!(
            "### Success\nPluralKit module setup for {}",
            system.name.map_or_else(
                || format!("`{}`", system.id),
                |system_name| format!("{} (`{}`)", system_name, system.id)
            )
        ),
    )
    .await?;

    Ok(())
}
