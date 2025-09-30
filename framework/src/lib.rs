use std::sync::Arc;

use twilight_gateway::Event;
use twilight_model::gateway::payload::incoming::InteractionCreate;

pub use context::{Context, EventContext, InteractionContext};
pub use framework::Framework;
pub use metadata::Metadata;
pub use module::{Module, builder::ModuleBuilder, registry::Registry};

pub mod context;
pub mod framework;
pub mod handler;
pub mod interaction;
pub mod macros;
pub mod metadata;
pub mod module;
pub mod scheduler;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn handle_interaction<T: Clone + Send + Sync + 'static>(
    event: InteractionCreate,
    context: Context<T>,
    meta: &Metadata,
    registry: &Registry<T>,
) -> Result<(), Error> {
    tracing::info!("interaction");

    match interaction::parse(&event, meta.clone(), context) {
        Ok(InteractionContext::Command(ctx)) => {
            let Some(command) = registry.find_command(&ctx.name) else {
                return Err(format!("unknown command /{}", ctx.name).into());
            };

            if let Err(err) = command.run(ctx.clone()).await {
                return Err(format!("error running command /{}: {}", ctx.name, err).into());
            }
        }
        Ok(InteractionContext::ComponentInteraction(ctx)) => {
            let Some(component_interaction) = registry.components.get(&ctx.interaction.custom_id)
            else {
                return Err(format!(
                    "no handler for component interaction {}",
                    ctx.interaction.custom_id
                )
                .into());
            };

            if let Err(err) = component_interaction.run(ctx.clone()).await {
                return Err(format!(
                    "error handling component interaction {}: {}",
                    ctx.interaction.custom_id, err
                )
                .into());
            }
        }
        Ok(InteractionContext::Modal(_modal_context)) => {
            todo!()
        }
        Err(err) => return Err(format!("error handling interaction: {}", err).into()),
    };

    Ok(())
}

pub async fn handle<T: Clone + Send + Sync + 'static>(
    meta: Metadata,
    ctx: Context<T>,
    registry: &Registry<T>,
    event: Event,
) {
    if let twilight_gateway::Event::InteractionCreate(event) = event.clone()
        && let Err(err) = handle_interaction(*event, ctx.clone(), &meta, registry).await
    {
        tracing::warn!(err);
    }

    if let Some(handlers) = registry.events.get(&event.kind()) {
        tracing::info!("running event handlers for {:?}", event.kind());

        for handler in handlers {
            let event_ctx = EventContext {
                meta: meta.clone(),
                application_id: ctx.application_id,
                client: Arc::clone(&ctx.client),
                services: Arc::clone(&ctx.services),

                event: event.clone(),
            };

            if let Err(err) = handler.run(event_ctx).await {
                tracing::warn!("error running event handler {}: {}", handler.uuid, err);
            }
        }
    }
}
