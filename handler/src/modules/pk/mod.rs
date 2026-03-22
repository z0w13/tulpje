use twilight_model::{
    application::{command::CommandType, interaction::InteractionContextType},
    guild::Permissions,
};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    Module, ModuleBuilder, handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder},
};

use crate::context::Services;

pub mod commands;
pub mod db;
pub mod fronters;
pub mod notify;
pub mod roles;
pub mod util;

pub fn build() -> Module<Services> {
    // define metrics
    metrics::describe_counter!("pk:tracked-systems", "Systems Tracked");
    metrics::describe_counter!("pk:notifications", "Front Notification Stats");
    metrics::describe_counter!("pk:front-category", "Front Category Stats");

    ModuleBuilder::<Services>::new("pluralkit")
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
                        .handler(handler_func!(commands::setup_pk)),
                )
                .subcommand(
                    SubCommandBuilder::new("update-member-roles", "update the member roles")
                        .option(StringBuilder::new("token", "(optional) PluralKit token"))
                        .handler(handler_func!(roles::update_member_roles)),
                )
                .group(fronters::commands())
                .group(notify::commands()),
        )
        // tasks
        .task(
            "pk:update-fronters",
            "0 * * * * *", // every minute
            handler_func!(fronters::tasks::update_fronters),
        )
        .build()
}
