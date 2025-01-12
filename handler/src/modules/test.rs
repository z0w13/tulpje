use tulpje_framework::{handler_func, Error, Module, ModuleBuilder};
use twilight_model::application::command::CommandType;
use twilight_util::builder::command::{
    CommandBuilder, StringBuilder, SubCommandBuilder, SubCommandGroupBuilder,
};

use crate::context::{CommandContext, Services};

pub(crate) fn build() -> Module<Services> {
    ModuleBuilder::<Services>::new("test")
        .command(
            CommandBuilder::new("test", "test command", CommandType::ChatInput)
                .option(
                    SubCommandBuilder::new("sub", "test subcommand")
                        .option(StringBuilder::new("who", "test").build()),
                )
                .option(
                    SubCommandGroupBuilder::new("group", "test group")
                        .subcommands(vec![SubCommandBuilder::new("sub", "test group subcommand")]),
                )
                .build(),
            vec![
                ("test sub", handler_func!(cmd_subcommand)),
                ("test group sub", handler_func!(cmd_group_subcommand)),
            ],
        )
        .build()
}

pub async fn cmd_subcommand(ctx: CommandContext) -> Result<(), Error> {
    tracing::debug!(?ctx.options);
    ctx.reply("subcommand").await?;
    Ok(())
}
pub async fn cmd_group_subcommand(ctx: CommandContext) -> Result<(), Error> {
    ctx.reply("group subcommand").await?;
    Ok(())
}
