use twilight_model::{
    application::interaction::{
        InteractionData,
        application_command::{CommandDataOption, CommandOptionValue},
    },
    gateway::payload::incoming::InteractionCreate,
};

use crate::{Error, Metadata, context};

pub fn parse<T: Clone + Send + Sync>(
    event: &InteractionCreate,
    meta: Metadata,
    ctx: context::Context<T>,
) -> Result<context::InteractionContext<T>, Error> {
    match &event.data {
        Some(InteractionData::ApplicationCommand(command)) => {
            let (name, options) = extract_command(&command.name, &command.options, &mut Vec::new());
            Ok(context::InteractionContext::<T>::Command(
                context::CommandContext::from_context(
                    meta,
                    ctx,
                    event.clone(),
                    *command.clone(),
                    name,
                    options,
                ),
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
            context::ModalContext::from_context(ctx, meta, event.clone(), *data.clone()),
        )),
        Some(_) => Err(format!("unknown interaction type: {:?}", event.kind).into()),
        None => Err("no interaction data".into()),
    }
}

fn extract_command<'a>(
    name: &'a str,
    options: &'a [CommandDataOption],
    parents: &mut Vec<&'a str>,
) -> (String, &'a [CommandDataOption]) {
    parents.push(name);

    if let Some((name, options)) = options.iter().find_map(|opt| match opt.value {
        CommandOptionValue::SubCommand(ref sub_options)
        | CommandOptionValue::SubCommandGroup(ref sub_options) => Some((&opt.name, sub_options)),
        _ => None,
    }) {
        extract_command(name, options, parents)
    } else {
        (parents.join(" "), options)
    }
}
