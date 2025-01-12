use std::collections::{HashMap, HashSet};

use async_cron_scheduler::cron::Schedule;
use twilight_gateway::EventType;
use twilight_model::application::command::Command;

use super::{command_builder::CommandBuilder, Module};
use crate::handler::{
    command_handler::CommandHandler,
    component_interaction_handler::{ComponentInteractionFunc, ComponentInteractionHandler},
    event_handler::{EventFunc, EventHandler},
    task_handler::{TaskFunc, TaskHandler},
};

pub struct ModuleBuilder<T: Clone + Send + Sync> {
    name: String,
    guild_scoped: bool,

    commands: HashMap<String, CommandHandler<T>>,
    command_definitions: Vec<Command>,

    components: HashMap<String, ComponentInteractionHandler<T>>,
    events: HashMap<EventType, HashSet<EventHandler<T>>>,
    tasks: HashMap<String, TaskHandler<T>>,
}

impl<T: Clone + Send + Sync> ModuleBuilder<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            guild_scoped: false,

            commands: HashMap::new(),
            command_definitions: Vec::new(),

            components: HashMap::new(),
            events: HashMap::new(),
            tasks: HashMap::new(),
        }
    }

    #[must_use]
    pub fn build(self) -> Module<T> {
        Module {
            name: self.name,
            guild_scoped: self.guild_scoped,

            commands: self.commands,
            command_definitions: self.command_definitions,

            components: self.components,
            events: self.events,
            tasks: self.tasks,
        }
    }

    #[must_use]
    pub fn guild(mut self) -> Self {
        self.guild_scoped = true;
        self
    }

    #[must_use]
    pub fn command(mut self, command: CommandBuilder<T>) -> Self {
        self.command_definitions.push(command.clone().into());

        for group in &command.groups {
            for subcommand in &group.commands {
                let command_name = format!("{} {} {}", command.name, group.name, subcommand.name);
                let func = subcommand
                    .func
                    .unwrap_or_else(|| panic!("command '/{}' has no handler", command_name));

                self.commands.insert(
                    command_name.clone(),
                    CommandHandler {
                        module: self.name.clone(),
                        name: command_name,
                        func,
                    },
                );
            }
        }
        for subcommand in &command.subcommands {
            let command_name = format!("{} {}", command.name, subcommand.name);
            let func = subcommand
                .func
                .unwrap_or_else(|| panic!("command /{} has no handler", command_name));

            self.commands.insert(
                command_name.clone(),
                CommandHandler {
                    module: self.name.clone(),
                    name: command_name,
                    func,
                },
            );
        }

        if command.subcommands.is_empty() && command.groups.is_empty() {
            let func = command
                .func
                .unwrap_or_else(|| panic!("command /{} has no handler", command.name));

            self.commands.insert(
                command.name.clone(),
                CommandHandler {
                    module: self.name.clone(),
                    name: command.name,
                    func,
                },
            );
        }

        self
    }

    #[must_use]
    pub fn component(mut self, custom_id: &str, func: ComponentInteractionFunc<T>) -> Self {
        self.components.insert(
            custom_id.to_string(),
            ComponentInteractionHandler {
                module: self.name.clone(),
                custom_id: custom_id.to_string(),
                func,
            },
        );
        self
    }

    #[must_use]
    pub fn event(mut self, event: EventType, func: EventFunc<T>) -> Self {
        self.events.entry(event).or_default().insert(EventHandler {
            module: self.name.clone(),
            uuid: uuid::Uuid::now_v7().to_string(),
            event,
            func,
        });
        self
    }

    #[must_use]
    pub fn task(mut self, name: &str, schedule: &str, func: TaskFunc<T>) -> Self {
        self.tasks.insert(
            name.to_string(),
            TaskHandler {
                module: self.name.clone(),
                name: name.to_string(),
                cron: Schedule::try_from(schedule).expect("failed to parse cron expression"),
                func,
            },
        );
        self
    }
}
