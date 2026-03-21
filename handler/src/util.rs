use tulpje_shared::color::{self, Color};

use twilight_model::channel::message::Component;
use twilight_util::builder::message::{ContainerBuilder, TextDisplayBuilder};

pub(crate) fn message(color: &Color, text: &str) -> Component {
    ContainerBuilder::new()
        .accent_color(Some(color.0))
        .component(TextDisplayBuilder::new(text).build())
        .build()
        .into()
}

pub(crate) fn success_message(text: &str) -> Component {
    message(&color::roles::GREEN, text)
}

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
