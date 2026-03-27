use tulpje_framework::Error;

use crate::{
    context::CommandContext,
    modules::pk::{
        db as pk_db,
        fronters::shared::handle_private_front,
        notify::{db, shared::resolve_system_from_reference},
        util::handle_system_ref,
    },
    responses,
};

const MAX_FOLLOW_COUNT: usize = 50;
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

    // parse the system reference the user provided
    let Some(system_ref) = handle_system_ref(&ctx, &ctx.get_arg_string("id")?).await? else {
        return Ok(());
    };

    // try to get the system from PluralKit or the database
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

    // handle already following
    if db::does_guild_follow(&ctx.services.db, guild.id, system.uuid).await? {
        responses::info(
            &ctx,
            &format!(
                "You're already following `{}`",
                system.name.unwrap_or(system.id)
            ),
        )
        .await?;

        return Ok(());
    }

    // handle follow limit
    if db::get_guild_follow_count(&ctx.services.db, guild.id).await? >= MAX_FOLLOW_COUNT {
        responses::error(
            &ctx,
            &format!("### Error\nYou've hit the follow limit of {MAX_FOLLOW_COUNT} please consider unfollowing some systems"),
        )
        .await?;

        return Ok(());
    }

    // inform user if the system they're trying to follow has a private front
    if handle_private_front(
        &ctx,
        system_ref.clone(),
        &format!(
            "Front for system `{}` is private",
            system.name.clone().unwrap_or_else(|| system.id.clone())
        ),
    )
    .await?
    {
        return Ok(());
    }

    // update the system in our database and add it to the watch list
    pk_db::update_system(&ctx.services.db, &system).await?;
    db::add_notify_system(&ctx.services.db, guild.id, system.uuid).await?;

    responses::success(
        &ctx,
        &format!(
            "`{}` added to notification list",
            system.name.unwrap_or(system.id),
        ),
    )
    .await?;

    Ok(())
}
