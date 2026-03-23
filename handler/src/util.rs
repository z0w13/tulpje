use tulpje_framework::Error;
use tulpje_shared::color::{self, Color};

use twilight_model::channel::message::{Component, MessageFlags};
use twilight_util::builder::message::{ContainerBuilder, TextDisplayBuilder};

use crate::context::CommandContext;

pub(crate) async fn success_response(ctx: &CommandContext, text: &str) -> Result<(), Error> {
    response(ctx, &color::roles::GREEN, text).await
}

pub(crate) async fn error_response(ctx: &CommandContext, text: &str) -> Result<(), Error> {
    response(ctx, &color::roles::RED, text).await
}

pub(crate) async fn response(ctx: &CommandContext, color: &Color, text: &str) -> Result<(), Error> {
    ctx.interaction()
        .update_response(&ctx.event.token)
        .flags(MessageFlags::IS_COMPONENTS_V2)
        .components(Some(&[message(color, text)]))
        .await?;

    Ok(())
}

pub(crate) fn message(color: &Color, text: &str) -> Component {
    ContainerBuilder::new()
        .accent_color(Some(color.0))
        .component(TextDisplayBuilder::new(text).build())
        .build()
        .into()
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn success_message(text: &str) -> Component {
    message(&color::roles::GREEN, text)
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn error_message(text: &str) -> Component {
    message(&color::roles::RED, text)
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn warning_message(text: &str) -> Component {
    message(&color::roles::ORANGE, text)
}

#[expect(dead_code, reason = "useful utility function we want to keep")]
pub(crate) fn info_message(text: &str) -> Component {
    message(&color::roles::BLUE, text)
}
