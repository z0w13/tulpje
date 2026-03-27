use std::{future::Future, pin::Pin};

use twilight_model::channel::message::MessageFlags;
use twilight_util::builder::message::{ContainerBuilder, TextDisplayBuilder};

use super::super::context::CommandContext;

use crate::{Error, color};

pub(crate) type CommandFunc<T> =
    fn(CommandContext<T>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;

#[derive(Clone)]
pub struct CommandHandler<T: Clone + Send + Sync> {
    pub module: String,
    pub name: String,
    pub func: CommandFunc<T>,
}

impl<T: Clone + Send + Sync> CommandHandler<T> {
    pub async fn run(&self, ctx: CommandContext<T>) -> Result<(), Error> {
        // TODO: More elegant way of handling command errors
        // TODO: Test if errors work in DMs
        if let Err(err) = (self.func)(ctx.clone()).await {
            tracing::error!(
                "error during command {}, sending reference to client: {}",
                self.name,
                err
            );

            if let Some(chan) = &ctx.event.channel {
                ctx.client
                    .create_message(chan.id)
                    .flags(MessageFlags::IS_COMPONENTS_V2)
                    .components(&[ContainerBuilder::new()
                        .accent_color(Some(color::roles::RED.0))
                        .component(
                            // TODO: Better way to handle extra error info than, whatever this is
                            TextDisplayBuilder::new(format!(
                                "### Internal Error\n{}\n**Error Code**\n```{}```",
                                std::env::var("TULPJE_EXTRA_ERROR_MESSAGE").unwrap_or_default(),
                                ctx.meta.uuid
                            ))
                            .build(),
                        )
                        .build()
                        .into()])
                    .await?;
            } else {
                tracing::warn!(event = ?ctx.event, "channel on event was empty, can't send error");
            }
        }

        Ok(())
    }
}
