use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::{
        db::{self as pk_db},
        notify::{db, shared::resolve_system_from_reference},
        util::handle_system_ref,
    },
    responses,
};

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    // check whether notification channel is set up
    if db::get_notify_channel(&ctx.services.db, guild.id)
        .await?
        .is_none()
    {
        responses::error(
            &ctx,
            "Notification channel not set-up please run `/pk notify setup` first",
        )
        .await?;

        return Ok(());
    }

    let Some(system_ref) = handle_system_ref(&ctx, &ctx.get_arg_string("id")?).await? else {
        return Ok(());
    };
    let Some(system) =
        resolve_system_from_reference(&system_ref, &ctx.services.pk, &ctx.services.db).await?
    else {
        responses::error(&ctx, &format!(
            "Couldn't find system `{}`, are you sure you're following them and that you spelled it correctly?",
            String::from(system_ref),
        ))
        .await?;
        return Ok(());
    };

    // update stored system info
    // TODO: Don't update if we fetched from the database instead of PluralKit
    pk_db::update_system(&ctx.services.db, &system).await?;

    // handle not following
    if !db::does_guild_follow(&ctx.services.db, guild.id, system.uuid).await? {
        responses::info(
            &ctx,
            &format!("You don't follow `{}`", system.name.unwrap_or(system.id)),
        )
        .await?;

        return Ok(());
    }

    db::remove_notify_system(&ctx.services.db, guild.id, system.uuid).await?;

    responses::success(
        &ctx,
        &format!(
            "{} removed from notification list",
            system.name.unwrap_or(system.id),
        ),
    )
    .await?;

    Ok(())
}
