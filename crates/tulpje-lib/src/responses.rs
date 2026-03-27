use twilight_model::{
    channel::message::MessageFlags,
    id::{Id, marker::ChannelMarker},
};

use crate::{context::CommandContext, util::message};
use tulpje_framework::{Error, color};

pub async fn with_color(
    ctx: &CommandContext,
    color: color::Color,
    text: &str,
) -> Result<(), Error> {
    ctx.interaction()
        .update_response(&ctx.event.token)
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(Some(&[message(color, text)]))
        .await?;

    Ok(())
}

pub async fn success(ctx: &CommandContext, text: &str) -> Result<(), Error> {
    with_color(ctx, color::roles::GREEN, text).await
}

pub async fn error(ctx: &CommandContext, text: &str) -> Result<(), Error> {
    with_color(ctx, color::roles::RED, text).await
}

pub async fn info(ctx: &CommandContext, text: &str) -> Result<(), Error> {
    with_color(ctx, color::roles::BLUE, text).await
}

pub async fn channel_not_found(ctx: &CommandContext, id: Id<ChannelMarker>) -> Result<(), Error> {
    error(
        ctx,
        &format!(
            "### Error\nCouldn't find channel, are you sure it's in this server and the bot can access it?\n\nChannel ID: `{id}`",
        ),
    )
    .await?;

    Ok(())
}
