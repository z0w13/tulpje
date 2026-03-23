use pkrs_fork::model::PkId;
use reqwest::StatusCode;
use tulpje_framework::Error;

use super::db;
use crate::{context::CommandContext, responses};

// TODO: command to see current settings

pub async fn setup_pk(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    ctx.defer_ephemeral().await?;

    let user_id = ctx.event.author_id().ok_or("no author?")?;
    let system_id = ctx.get_arg_string("system_id")?;

    // sanitise and validate system id
    let system_id = system_id.trim().replace("-", "").to_lowercase();
    if !system_id.chars().all(|c| char::is_ascii_alphabetic(&c)) {
        responses::error(&ctx, &format!("### Error\nInvalid system id `{system_id}`")).await?;
        return Ok(());
    }

    db::save_guild_settings(&ctx.services.db, guild.id, user_id, &system_id).await?;

    // TODO: fix pkrs to actually handle 404s correctly
    let system = match ctx.services.pk.get_system(&PkId(system_id.clone())).await {
        Ok(system) => system,
        Err(err)
            if err
                .status()
                .is_some_and(|status| status == StatusCode::NOT_FOUND) =>
        {
            responses::error(
                &ctx,
                &format!("### Error\nCouldn't find system `{system_id}`"),
            )
            .await?;

            return Ok(());
        }
        Err(err) => return Err(err.into()),
    };

    // Inform user of success
    responses::success(
        &ctx,
        &format!(
            "### Success\nPluralKit module setup for {}",
            system.name.map_or_else(
                || format!("`{}`", system_id),
                |system_name| format!("{} (`{}`)", system_name, system_id)
            )
        ),
    )
    .await?;

    Ok(())
}
