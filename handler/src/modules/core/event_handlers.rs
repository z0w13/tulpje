use tulpje_framework::Error;
use twilight_gateway::Event;

use crate::{context::EventContext, modules::core::db};

pub async fn guild_create(ctx: EventContext) -> Result<(), Error> {
    let Event::GuildCreate(guild_create) = &ctx.event else {
        unreachable!()
    };

    let guild_id = guild_create.id();
    tracing::debug!("registering commands for guild {}", guild_id);

    let commands: Vec<_> = db::guild_modules(&ctx.services.db, guild_id)
        .await?
        .iter()
        .filter_map(|name| ctx.services.registry.module_commands(&name))
        .flatten()
        .collect();

    ctx.client
        .interaction(ctx.application_id)
        .set_guild_commands(guild_id, &commands)
        .await?;

    Ok(())
}
