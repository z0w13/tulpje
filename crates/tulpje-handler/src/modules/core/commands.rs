use tulpje_framework::Error;

use super::{db, set_guild_commands_for_guild};
use crate::context::CommandContext;

pub(crate) async fn enable(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    let module = ctx.get_arg_string("module")?;
    if !ctx.services.registry.guild_module_names().contains(&module) {
        ctx.reply(format!("invalid module {}", module)).await?;
        return Ok(());
    }

    db::enable_module(&ctx.services.db, guild.id, &module).await?;
    set_guild_commands_for_guild(
        &db::guild_modules(&ctx.services.db, guild.id).await?,
        guild.id,
        ctx.interaction(),
        &ctx.services.registry,
    )
    .await?;

    ctx.reply(format!("{} enabled", module)).await?;

    Ok(())
}

pub(crate) async fn disable(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    let module = ctx.get_arg_string("module")?;
    if !ctx.services.registry.guild_module_names().contains(&module) {
        ctx.reply(format!("invalid module {}", module)).await?;
        return Ok(());
    }

    db::disable_module(&ctx.services.db, guild.id, &module).await?;
    set_guild_commands_for_guild(
        &db::guild_modules(&ctx.services.db, guild.id).await?,
        guild.id,
        ctx.interaction(),
        &ctx.services.registry,
    )
    .await?;

    ctx.reply(format!("{} disabled", module)).await?;

    Ok(())
}

pub(crate) async fn modules(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };

    let modules = db::guild_modules(&ctx.services.db, guild.id).await?;
    let available: Vec<String> = ctx
        .services
        .registry
        .guild_module_names()
        .into_iter()
        .filter(|m| !modules.contains(m))
        .collect();

    ctx.reply(format!(
        "**Enabled: {}**\nAvailable: {}",
        modules.join(", "),
        available.join(", ")
    ))
    .await?;

    Ok(())
}
