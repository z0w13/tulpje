use tulpje_framework::Error;
use tulpje_lib::context::EventContext;
use twilight_gateway::Event;
use twilight_model::id::{Id, marker::GuildMarker};

use crate::db;

pub async fn guild_create(ctx: EventContext) -> Result<(), Error> {
    let Event::GuildCreate(guild_create) = &ctx.event else {
        unreachable!()
    };

    let guild_id = guild_create.id();

    db::touch_guild(&ctx.services.db, guild_id).await?;
    register_commands(&ctx, guild_id).await?;

    Ok(())
}
pub async fn guild_delete(ctx: EventContext) -> Result<(), Error> {
    let Event::GuildDelete(guild_delete) = &ctx.event else {
        unreachable!()
    };

    if guild_delete.unavailable.is_some() {
        tracing::debug!(
            "GuildDelete was because guild {} is unavailable, ignoring",
            guild_delete.id,
        );
        return Ok(());
    }

    tracing::info!("was removed from guild `{}`", guild_delete.id);
    db::leave_guild(&ctx.services.db, guild_delete.id).await?;

    Ok(())
}

pub async fn register_commands(ctx: &EventContext, guild_id: Id<GuildMarker>) -> Result<(), Error> {
    tracing::debug!("registering commands for guild {}", guild_id);

    let commands: Vec<_> = db::guild_modules(&ctx.services.db, guild_id)
        .await?
        .iter()
        .filter_map(|name| ctx.services.registry.module_commands(name))
        .flatten()
        .collect();

    ctx.client
        .interaction(ctx.application_id)
        .set_guild_commands(guild_id, &commands)
        .await?;

    Ok(())
}
