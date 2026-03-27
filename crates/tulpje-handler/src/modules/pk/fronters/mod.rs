use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    handler_func,
    module::command_builder::{SubCommandBuilder, SubCommandGroupBuilder},
};

use crate::context::Services;

pub(crate) mod db;
pub(crate) mod setup;
pub(crate) mod shared;
pub(crate) mod tasks;
pub(crate) mod update;

pub(crate) fn commands() -> SubCommandGroupBuilder<Services> {
    SubCommandGroupBuilder::new("fronters", "fronter related commands")
        .subcommand(
            SubCommandBuilder::new("setup", "set-up fronter channels")
                .option(
                    StringBuilder::new("title", "title of the fronters category")
                        .max_length(100)
                        .required(true),
                )
                .handler(handler_func!(setup::handle)),
        )
        .subcommand(
            SubCommandBuilder::new("update", "manually update fronter channels")
                .handler(handler_func!(update::handle)),
        )
}
