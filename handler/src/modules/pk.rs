use twilight_model::{
    application::{command::CommandType, interaction::InteractionContextType},
    guild::Permissions,
};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder, SubCommandGroupBuilder},
    Module, ModuleBuilder,
};

use crate::context::Services;

pub mod commands;
pub mod db;
pub mod fronters;
pub mod roles;
pub mod util;

pub fn build() -> Module<Services> {
    ModuleBuilder::<Services>::new("pluralkit")
        .guild()
        // commands
        .command(
            CommandBuilder::new("pk", "PluralKit related commands", CommandType::ChatInput)
                .default_member_permissions(Permissions::MANAGE_GUILD)
                .contexts([InteractionContextType::Guild])
                .subcommand(
                    SubCommandBuilder::new("setup", "set-up the PluralKit module")
                        .option(
                            StringBuilder::new("system_id", "PluralKit system ID").required(true),
                        )
                        .option(StringBuilder::new("token", "(optional) PluralKit token"))
                        .handler(handler_func!(commands::setup_pk)),
                )
                .subcommand(
                    SubCommandBuilder::new("update-member-roles", "update the member roles")
                        .handler(handler_func!(roles::update_member_roles)),
                )
                .group(
                    SubCommandGroupBuilder::new("fronters", "fronter related commands")
                        .subcommand(
                            SubCommandBuilder::new("setup", "set-up fronter channels")
                                .option(
                                    StringBuilder::new("name", "Name of the fronters category")
                                        .max_length(100)
                                        .required(true),
                                )
                                .handler(handler_func!(fronters::commands::setup_fronters)),
                        )
                        .subcommand(
                            SubCommandBuilder::new("update", "manually update fronter channels")
                                .handler(handler_func!(fronters::commands::update_fronters)),
                        ),
                ),
        )
        // tasks
        .task(
            "pk:update-fronters",
            "0 * * * * *", // every minute
            handler_func!(fronters::tasks::update_fronters),
        )
        .build()
}
