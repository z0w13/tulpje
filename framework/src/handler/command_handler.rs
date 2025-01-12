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
        // can add more handling/parsing/etc here in the future
        (self.func)(ctx).await
    }
}
