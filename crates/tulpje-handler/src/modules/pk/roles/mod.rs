use tulpje_framework::{
    handler_func,
    module::command_builder::{SubCommandBuilder, SubCommandGroupBuilder},
};
use twilight_util::builder::command::StringBuilder;

use crate::context::Services;

mod update;

pub(crate) fn commands() -> SubCommandGroupBuilder<Services> {
    SubCommandGroupBuilder::new("roles", "manage member roles").subcommand(
        SubCommandBuilder::new(
            "update",
            "updates member roles to match the configured system",
        )
        .option(StringBuilder::new("token", "(optional) PluralKit token"))
        .handler(handler_func!(update::handle)),
    )
}
