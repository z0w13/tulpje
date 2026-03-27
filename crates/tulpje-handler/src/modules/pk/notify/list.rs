use tulpje_framework::Error;
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::modules::pk::{db as pk_db, notify::db};
use tulpje_lib::context::CommandContext;

pub(crate) async fn handle(ctx: CommandContext) -> Result<(), Error> {
    let Some(guild) = ctx.guild().await? else {
        unreachable!("command is guild_only");
    };
    ctx.defer().await?;

    let system_uuids = db::get_notify_systems(&ctx.services.db, guild.id).await?;
    let systems = pk_db::get_systems(&ctx.services.db, system_uuids).await?;

    let systems_is_empty = systems.is_empty();
    let systems_len = systems.len();

    let followed_systems_text = if !systems_is_empty {
        systems
            .into_iter()
            .map(|system| {
                if let Some(name) = system.name {
                    format!("`{:<6}` • {}", system.id, name)
                } else {
                    format!("`{:<6}`", system.id)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        "Not Following Anyone".to_string()
    };

    let mut builder = EmbedBuilder::new()
        .title(format!("Followed Systems In {}", guild.name))
        .description(followed_systems_text);

    if !systems_is_empty {
        let footer_text = if systems_len == 1 {
            "Following 1 system".to_string()
        } else {
            format!("Following {} systems", systems_len)
        };
        builder = builder.footer(EmbedFooterBuilder::new(footer_text));
    }

    ctx.interaction()
        .update_response(&ctx.event.token)
        .content(None)
        .embeds(Some(&[builder.validate()?.build()]))
        .await?;

    Ok(())
}
