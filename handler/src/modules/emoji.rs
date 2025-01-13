pub mod clone;
pub mod commands;
pub mod db;
pub mod event_handlers;
pub mod shared;

use twilight_gateway::EventType;
use twilight_model::{application::command::CommandType, guild::Permissions};
use twilight_util::builder::command::StringBuilder;

use tulpje_framework::{
    handler_func,
    module::command_builder::{CommandBuilder, SubCommandBuilder},
    Module, ModuleBuilder,
};

use crate::context::Services;

pub(crate) fn build() -> Module<Services> {
    ModuleBuilder::<Services>::new("emoji")
        // commands
        .command(
            CommandBuilder::new("emoji", "emoji related commands", CommandType::ChatInput)
                .default_member_permissions(Permissions::MANAGE_GUILD_EXPRESSIONS)
                .dm_permission(false)
                .subcommand(
                    SubCommandBuilder::new("stats", "stats for emojis in this server")
                        .option(
                            StringBuilder::new("sort", "How to sort the emojis").choices([
                                ("Most Used", "count_desc"),
                                ("Least Used", "count_asc"),
                                ("Most Recent", "date_desc"),
                                ("Least Recent", "date_asc"),
                            ]),
                        )
                        .handler(handler_func!(commands::cmd_emoji_stats)),
                )
                .subcommand(
                    SubCommandBuilder::new("clone", "clone an emoji to this server")
                        .option(StringBuilder::new("emoji", "emojis to clone").required(true))
                        .option(StringBuilder::new(
                            "new_name",
                            "new name (only if cloning a single emoji)",
                        ))
                        .option(StringBuilder::new("prefix", "prefix for new emoji(s)"))
                        .handler(handler_func!(clone::command)),
                )
                .subcommand(
                    SubCommandBuilder::new("maintenance", "remove deleted emojis from stats")
                        .handler(handler_func!(commands::cmd_emoji_maintenance)),
                ),
        )
        .command(
            CommandBuilder::new("Clone Emojis", "", CommandType::Message)
                .default_member_permissions(Permissions::MANAGE_GUILD_EXPRESSIONS)
                .dm_permission(false)
                .handler(handler_func!(clone::context_command)),
        )
        // component interactions
        .component(
            "emoji_stats_sort",
            handler_func!(commands::handle_emoji_stats_sort),
        )
        //// pagination
        .component(
            "emoji_stats_first_page",
            handler_func!(commands::handle_emoji_pagination),
        )
        .component(
            "emoji_stats_prev_page",
            handler_func!(commands::handle_emoji_pagination),
        )
        .component(
            "emoji_stats_next_page",
            handler_func!(commands::handle_emoji_pagination),
        )
        .component(
            "emoji_stats_last_page",
            handler_func!(commands::handle_emoji_pagination),
        )
        // event handlers
        .event(
            EventType::MessageCreate,
            handler_func!(event_handlers::handle_message),
        )
        .event(
            EventType::MessageUpdate,
            handler_func!(event_handlers::message_update),
        )
        .event(
            EventType::ReactionAdd,
            handler_func!(event_handlers::reaction_add),
        )
        .event(
            EventType::GuildCreate,
            handler_func!(event_handlers::guild_create),
        )
        .event(
            EventType::GuildEmojisUpdate,
            handler_func!(event_handlers::guild_emojis_update),
        )
        .build()
}
