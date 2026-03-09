use std::{future::Future, pin::Pin};

use super::super::context::CommandContext;

use crate::Error;

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
        if let Err(err) = (self.func)(ctx.clone()).await {
            tracing::info!(
                "error during command {}, sending to client ephemerally",
                self.name
            );
            // TODO: Keep internal state of whether we deferred, etc. so we can
            //       avoid doing this and ignoring the error
            if let Err(err) = ctx.defer_ephemeral().await {
                tracing::warn!("error on defer_ephemeral, can probably be ignored: {err}");
            }
            ctx.update(format!("{err}")).await?;

            return Err(err);
        }

        Ok(())
    }
}
