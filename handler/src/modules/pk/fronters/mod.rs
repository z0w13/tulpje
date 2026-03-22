use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    handler_func,
    module::command_builder::{SubCommandBuilder, SubCommandGroupBuilder},
};

use crate::context::Services;

pub(crate) mod commands;
pub(crate) mod db;
pub(crate) mod tasks;

pub(crate) fn commands() -> SubCommandGroupBuilder<Services> {
    SubCommandGroupBuilder::new("fronters", "fronter related commands")
        .subcommand(
            SubCommandBuilder::new("setup", "set-up fronter channels")
                .option(
                    StringBuilder::new("name", "Name of the fronters category")
                        .max_length(100)
                        .required(true),
                )
                .handler(handler_func!(commands::setup_fronters)),
        )
        .subcommand(
            SubCommandBuilder::new("update", "manually update fronter channels")
                .handler(handler_func!(commands::update_fronters)),
        )
}
