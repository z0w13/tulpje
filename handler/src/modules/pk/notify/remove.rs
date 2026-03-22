use pkrs_fork::client::PkClient;
use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::{
        db::{self as pk_db},
        notify::{db, shared::resolve_system_from_reference},
        util::SystemRef,
    },
    util::{error_response, success_response},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let ref_str = ctx.get_arg_string("id")?;
    let system_ref: SystemRef =
        match ref_str.parse() {
            Ok(system_ref) => system_ref,
            Err(_) => {
                error_response(&ctx, &format!(
                    "Invalid system reference `{ref_str}`, are you sure you entered it correctly?",
                )).await?;
                return Ok(());
            }
        };
    let Some(system) =
        resolve_system_from_reference(&system_ref, &PkClient::default(), &ctx.services.db).await?
    else {
        error_response(&ctx, &format!(
            "Couldn't find system `{}`, are you sure you're following them and that you spelled it correctly?",
            String::from(system_ref),
        ))
        .await?;
        return Ok(());
    };

    pk_db::update_system(&ctx.services.db, &system).await?;
    db::remove_notify_system(&ctx.services.db, guild.id, system.uuid).await?;
    pk_db::cleanup_system(&ctx.services.db, system.uuid).await?;

    success_response(
        &ctx,
        &format!(
            "{} removed from notification list",
            system.name.unwrap_or(system.id),
        ),
    )
    .await?;

    Ok(())
}
