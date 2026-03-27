use tulpje_framework::{
    handler_func,
    module::command_builder::{SubCommandBuilder, SubCommandGroupBuilder},
};
use twilight_util::builder::command::StringBuilder;

use tulpje_lib::context::Services;

mod add;
pub(super) mod db;
mod list;
mod remove;
mod setup;
mod shared;

pub(crate) fn commands() -> SubCommandGroupBuilder<Services> {
    SubCommandGroupBuilder::new("notify", "front change notify commands")
        .subcommand(
            SubCommandBuilder::new("setup", "set-up front change notifications")
                .option(
                    StringBuilder::new(
                        "channel",
                        "Channel to use for notifications (will be created if it doesn't exist)",
                    )
                    .max_length(100)
                    .required(true),
                )
                .handler(handler_func!(setup::handle)),
        )
        .subcommand(
            SubCommandBuilder::new("list", "list systems you have notifications enabled for")
                .handler(handler_func!(list::handle)),
        )
        .subcommand(
            SubCommandBuilder::new("add", "add a system from the notify list")
                .option(
                    StringBuilder::new("id", "discord id or system id/uuid of the system to add")
                        .max_length(36)
                        .required(true),
                )
                .handler(handler_func!(add::handle)),
        )
        .subcommand(
            SubCommandBuilder::new("remove", "remove a system from the notify list")
                .option(
                    StringBuilder::new(
                        "id",
                        "discord id or system id/uuid of the system to remove",
                    )
                    .max_length(36)
                    .required(true),
                )
                .handler(handler_func!(remove::handle)),
        )
}
