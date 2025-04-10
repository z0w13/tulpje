use twilight_model::application::{command::CommandType, interaction::InteractionContextType};

use tulpje_framework::{
    handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder},
    Module, ModuleBuilder,
};

use crate::context::Services;

mod commands;
mod redis;

pub(crate) fn build() -> Module<Services> {
    ModuleBuilder::<Services>::new("stats")
        .command(
            CommandBuilder::new("stats", "Bot stats", CommandType::ChatInput)
                .contexts([InteractionContextType::Guild])
                .handler(handler_func!(commands::stats)),
        )
        .command(
            // TODO: Lock this to a specific guild or list of guilds
            CommandBuilder::new("info", "various bot statistics", CommandType::ChatInput)
                .contexts([InteractionContextType::Guild])
                .subcommand(
                    SubCommandBuilder::new("shards", "bot shard stats")
                        .handler(handler_func!(commands::shards)),
                )
                .subcommand(
                    SubCommandBuilder::new("processes", "bot process stats")
                        .handler(handler_func!(commands::processes)),
                ),
        )
        .build()
}
