use twilight_model::{application::command::CommandType, guild::Permissions};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    handler_func, module::command_builder::CommandBuilder, Module, ModuleBuilder,
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
            CommandBuilder::new(
                "setup-pk",
                "set-up the PluralKit module",
                CommandType::ChatInput,
            )
            .default_member_permissions(Permissions::MANAGE_GUILD)
            .dm_permission(false)
            .option(StringBuilder::new("system_id", "PluralKit system ID").required(true))
            .option(StringBuilder::new("token", "(optional) PluralKit token"))
            .handler(handler_func!(commands::setup_pk)),
        )
        .command(
            CommandBuilder::new(
                "setup-fronters",
                "set-up fronter channels",
                CommandType::ChatInput,
            )
            .default_member_permissions(Permissions::MANAGE_GUILD)
            .dm_permission(false)
            .option(StringBuilder::new("name", "Name of the fronters category"))
            .handler(handler_func!(fronters::commands::setup_fronters)),
        )
        .command(
            CommandBuilder::new(
                "update-fronters",
                "manually update fronter channels",
                CommandType::ChatInput,
            )
            .default_member_permissions(Permissions::MANAGE_GUILD)
            .dm_permission(false)
            .handler(handler_func!(fronters::commands::update_fronters)),
        )
        .command(
            CommandBuilder::new(
                "update-member-roles",
                "update the member roles",
                CommandType::ChatInput,
            )
            .default_member_permissions(Permissions::MANAGE_GUILD)
            .dm_permission(false)
            .handler(handler_func!(roles::update_member_roles)),
        )
        // tasks
        .task(
            "pk:update-fronters",
            "0 * * * * *", // every minute
            handler_func!(fronters::tasks::update_fronters),
        )
        .build()
}
