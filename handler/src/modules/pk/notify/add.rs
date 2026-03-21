use pkrs_fork::client::PkClient;
use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::{
        db as pk_db,
        notify::{db, shared::resolve_system_from_reference},
        util::SystemRef,
    },
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let system_ref: SystemRef = ctx.get_arg_string("id")?.parse()?;
    let Some(system) =
        resolve_system_from_reference(system_ref, &PkClient::default(), &ctx.services.db).await?
    else {
        ctx.update("Couldn't find system '{system_ref}', are you sure you're following them and that you spelled it correctly?").await?;
        return Ok(());
    };

    pk_db::update_system(&ctx.services.db, &system).await?;
    db::add_notify_system(&ctx.services.db, guild.id, system.uuid).await?;

    ctx.update(format!(
        "SUCCESS: will notify you if `{}` front changes",
        system.name.unwrap_or(system.id)
    ))
    .await?;

    Ok(())
}
