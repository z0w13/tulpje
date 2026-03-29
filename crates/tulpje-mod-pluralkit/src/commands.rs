use pkrs_fork::model::PkId;
use reqwest::StatusCode;
use tulpje_framework::Error;

use super::{
    db::{self, ModPkSystem},
    util::handle_system_ref,
};
use tulpje_lib::{context::CommandContext, responses};

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

    let system: ModPkSystem = match ctx
        .services
        .pk
        .get_system(&PkId(system_ref.clone().into()))
        .await
    {
        Ok(system) => system.into(),
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

    tulpje_lib::db::touch_guild(&ctx.services.db, guild.id).await?;
    db::update_system(&ctx.services.db, &system).await?;
    db::save_guild_settings(&ctx.services.db, guild.id, user_id, system.uuid).await?;

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
