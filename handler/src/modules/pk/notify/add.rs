use pkrs_fork::client::PkClient;
use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::{
        db as pk_db,
        notify::{
            db,
            shared::{handle_system_ref, resolve_system_from_reference},
        },
    },
    util::{error_response, success_response},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    // parse the system reference the user provided
    let Some(system_ref) = handle_system_ref(&ctx, &ctx.get_arg_string("id")?).await? else {
        return Ok(());
    };

    // try to get the system from PluralKit or the database
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

    // update the system in our database and add it to the watch list
    pk_db::update_system(&ctx.services.db, &system).await?;
    db::add_notify_system(&ctx.services.db, guild.id, system.uuid).await?;

    success_response(
        &ctx,
        &format!(
            "`{}` added to notification list",
            system.name.unwrap_or(system.id),
        ),
    )
    .await?;

    Ok(())
}
