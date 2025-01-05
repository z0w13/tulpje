use tulpje_shared::DiscordEventMeta;
use twilight_model::{
    application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate,
};

use super::context;
use crate::context::Context;
use crate::Error;

pub fn parse<T: Clone + Send + Sync>(
    event: &InteractionCreate,
    meta: DiscordEventMeta,
    ctx: Context<T>,
) -> Result<context::InteractionContext<T>, Error> {
    match &event.data {
        Some(InteractionData::ApplicationCommand(command)) => {
            Ok(context::InteractionContext::<T>::Command(
                context::CommandContext::from_context(meta, ctx, event.clone(), *command.clone()),
            ))
        }
        Some(InteractionData::MessageComponent(interaction)) => {
            Ok(context::InteractionContext::<T>::ComponentInteraction(
                context::ComponentInteractionContext::from_context(
                    ctx,
                    meta,
                    event.clone(),
                    *interaction.clone(),
                ),
            ))
        }
        Some(InteractionData::ModalSubmit(data)) => Ok(context::InteractionContext::<T>::Modal(
            context::ModalContext::from_context(ctx, meta, event.clone(), data.clone()),
        )),
        Some(_) => Err(format!("unknown interaction type: {:?}", event.kind).into()),
        None => Err("no interaction data".into()),
    }
}
