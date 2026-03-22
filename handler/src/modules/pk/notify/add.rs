use pkrs_fork::client::PkClient;
use tulpje_framework::Error;
use twilight_model::channel::message::MessageFlags;

use crate::{
    context::CommandContext,
    modules::pk::{
        db as pk_db,
        notify::{db, shared::resolve_system_from_reference},
        util::SystemRef,
    },
    util::{error_message, success_message},
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let ref_str = ctx.get_arg_string("id")?;
    let system_ref: SystemRef = match ref_str.parse() {
        Ok(system_ref) => system_ref,
        Err(_) => {
            ctx.interaction()
                .update_response(&ctx.event.token)
                .flags(MessageFlags::IS_COMPONENTS_V2)
                .components(Some(&[error_message(&format!(
                    "Invalid system reference `{ref_str}`, are you sure you entered it correctly?",
                ))]))
                .await?;
            return Ok(());
        }
    };
    let Some(system) =
        resolve_system_from_reference(&system_ref, &PkClient::default(), &ctx.services.db).await?
    else {
        ctx.interaction()
        .update_response(&ctx.event.token)
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(Some(&[error_message(&format!(
            "Couldn't find system `{}`, are you sure you're following them and that you spelled it correctly?",
            String::from(system_ref),
        ))]))
        .await?;

        return Ok(());
    };

    pk_db::update_system(&ctx.services.db, &system).await?;
    db::add_notify_system(&ctx.services.db, guild.id, system.uuid).await?;

    ctx.interaction()
        .update_response(&ctx.event.token)
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(Some(&[success_message(&format!(
            "`{}` added to notification list",
            system.name.unwrap_or(system.id),
        ))]))
        .await?;

    Ok(())
}
