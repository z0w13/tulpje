use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::{
        db as pk_db,
        notify::{db, shared::resolve_system_from_reference},
    },
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let system_id_uuid_or_discord_id = ctx.get_arg_string("id")?;
    let system = resolve_system_from_reference(system_id_uuid_or_discord_id).await?;

    pk_db::update_system(&ctx.services.db, &system).await?;
    db::add_notify_system(&ctx.services.db, guild.id, system.uuid).await?;

    ctx.update(format!(
        "SUCCESS: will notify you if `{}` front changes",
        system.name.unwrap_or(system.id.0)
    ))
    .await?;

    Ok(())
}
