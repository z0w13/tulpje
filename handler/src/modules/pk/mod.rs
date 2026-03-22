use twilight_model::{
    application::{command::CommandType, interaction::InteractionContextType},
    guild::Permissions,
};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    Module, ModuleBuilder, handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder, SubCommandGroupBuilder},
};

use crate::context::Services;

pub mod commands;
pub mod db;
pub mod fronters;
pub mod notify;
pub mod roles;
pub mod util;

pub fn build() -> Module<Services> {
    setup_metrics();

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
                )
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

fn setup_metrics() {
    metrics::describe_counter!("pk:tracked-systems", "Systems Tracked");

    metrics::describe_counter!("pk:notifications", "Front Notification Stats");
    metrics::counter!("pk:notifications", "type" => "total").absolute(0);
    metrics::counter!("pk:notifications", "type" => "category-missing").absolute(0);
    metrics::counter!("pk:notifications", "type" => "error").absolute(0);
    metrics::counter!("pk:notifications", "type" => "success").absolute(0);

    metrics::describe_counter!("pk:front-category", "Front Category Stats");
    metrics::counter!("pk:front-category", "type" => "total").absolute(0);
    metrics::counter!("pk:front-category", "type" => "category-missing").absolute(0);
    metrics::counter!("pk:front-category", "type" => "error").absolute(0);
    metrics::counter!("pk:front-category", "type" => "success").absolute(0);
}
