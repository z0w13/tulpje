use twilight_http::client::InteractionClient;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::InteractionContextType,
    },
    guild::Permissions,
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder},
    Error, Module, ModuleBuilder, Registry,
};

use crate::context::Services;

mod commands;
pub(crate) mod db;

pub(crate) fn build(registry: &Registry<Services>) -> Module<Services> {
    let guild_module_choices: Vec<(String, String)> = registry
        .guild_module_names()
        .into_iter()
        .map(|m| (m.clone(), m))
        .collect();

    ModuleBuilder::<Services>::new("core")
        .command(
            CommandBuilder::new("mod", "module management", CommandType::ChatInput)
                .default_member_permissions(Permissions::MANAGE_GUILD)
                .contexts([InteractionContextType::Guild])
                .subcommand(
                    SubCommandBuilder::new("enable", "enable a module for this server")
                        .option(
                            StringBuilder::new("module", "The module to enable")
                                .choices(guild_module_choices.clone())
                                .required(true),
                        )
                        .handler(handler_func!(commands::enable)),
                )
                .subcommand(
                    SubCommandBuilder::new("disable", "disable a module for this server")
                        .option(
                            StringBuilder::new("module", "The module to disable")
                                .choices(guild_module_choices)
                                .required(true),
                        )
                        .handler(handler_func!(commands::disable)),
                )
                .subcommand(
                    SubCommandBuilder::new("list", "list enabled and available server modules")
                        .handler(handler_func!(commands::modules)),
                ),
        )
        .build()
}

pub(crate) async fn set_guild_commands_for_guild(
    modules: &[String],
    guild_id: Id<GuildMarker>,
    interaction: InteractionClient<'_>,
    registry: &Registry<Services>,
) -> Result<(), Error> {
    let commands: Vec<Command> = modules
        .iter()
        .filter_map(|module| registry.module_commands(module))
        .flatten()
        .collect();

    tracing::debug!(
        "setting commands [{}] for guild {}",
        commands
            .iter()
            .map(|cmd| cmd.name.clone())
            .collect::<Vec<String>>()
            .join(", "),
        guild_id
    );

    interaction.set_guild_commands(guild_id, &commands).await?;

    Ok(())
}
