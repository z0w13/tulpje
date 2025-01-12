use std::collections::{HashMap, HashSet};

use twilight_gateway::EventType;
use twilight_model::application::{
    command::Command,
    interaction::application_command::{CommandData, CommandDataOption, CommandOptionValue},
};

use super::Module;
use crate::handler::{
    command_handler::CommandHandler, component_interaction_handler::ComponentInteractionHandler,
    event_handler::EventHandler, task_handler::TaskHandler,
};

#[derive(Clone)]
#[expect(
    clippy::partial_pub_fields,
    reason = "we need 'tasks' to be public for now to start the task scheduler"
)]
pub struct Registry<T: Clone + Send + Sync> {
    modules: HashMap<String, Module<T>>,

    pub(crate) command_handlers: HashMap<String, CommandHandler<T>>,
    pub(crate) components: HashMap<String, ComponentInteractionHandler<T>>,
    pub(crate) events: HashMap<EventType, HashSet<EventHandler<T>>>,
    pub tasks: HashMap<String, TaskHandler<T>>,
}

impl<T: Clone + Send + Sync> Registry<T> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            command_handlers: HashMap::new(),
            components: HashMap::new(),
            events: HashMap::new(),
            tasks: HashMap::new(),
        }
    }

    pub fn register(&mut self, module: Module<T>) {
        self.command_handlers
            .extend(module.command_handlers.clone());
        self.components.extend(module.components.clone());
        self.events.extend(module.events.clone());
        self.tasks.extend(module.tasks.clone());

        self.modules.insert(module.name.clone(), module);
    }

    pub fn global_commands(&self) -> Vec<Command> {
        self.modules
            .values()
            .filter(|m| !m.guild_scoped) // filter out guild scoped modules
            .flat_map(|m| m.command_definitions.values().cloned())
            .collect()
    }

    pub fn module_commands(&self, module: &str) -> Option<Vec<Command>> {
        Some(
            self.modules
                .get(module)?
                .command_definitions
                .values()
                .cloned()
                .collect(),
        )
    }

    pub fn find_command<'a>(
        &self,
        data: &'a CommandData,
    ) -> Option<(&CommandHandler<T>, &'a [CommandDataOption])> {
        let (name, options) = extract_command(&data.name, &data.options, &mut Vec::new());
        self.command_handlers
            .get(&name)
            .map(|command| (command, options))
    }

    pub fn guild_module_names(&self) -> Vec<String> {
        self.modules
            .values()
            .filter(|m| m.guild_scoped)
            .map(|m| m.name.clone())
            .collect()
    }
}

impl<T: Clone + Send + Sync> Default for Registry<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_command<'a>(
    name: &'a str,
    options: &'a [CommandDataOption],
    parents: &mut Vec<&'a str>,
) -> (String, &'a [CommandDataOption]) {
    parents.push(name);

    if let Some((name, options)) = options.iter().find_map(|opt| match opt.value {
        CommandOptionValue::SubCommand(ref subopts)
        | CommandOptionValue::SubCommandGroup(ref subopts) => Some((&opt.name, subopts)),
        _ => None,
    }) {
        extract_command(name, options, parents)
    } else {
        (parents.join(" "), options)
    }
}
