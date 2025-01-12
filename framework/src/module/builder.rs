use std::collections::{HashMap, HashSet};

use async_cron_scheduler::cron::Schedule;
use twilight_gateway::EventType;
use twilight_model::application::command::Command;

use super::Module;
use crate::handler::{
    command_handler::{CommandFunc, CommandHandler},
    component_interaction_handler::{ComponentInteractionFunc, ComponentInteractionHandler},
    event_handler::{EventFunc, EventHandler},
    task_handler::{TaskFunc, TaskHandler},
};

pub struct ModuleBuilder<T: Clone + Send + Sync> {
    name: String,
    guild_scoped: bool,

    command_definitions: HashMap<String, Command>,
    command_handlers: HashMap<String, CommandHandler<T>>,

    components: HashMap<String, ComponentInteractionHandler<T>>,
    events: HashMap<EventType, HashSet<EventHandler<T>>>,
    tasks: HashMap<String, TaskHandler<T>>,
}

impl<T: Clone + Send + Sync> ModuleBuilder<T> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            guild_scoped: false,

            command_definitions: HashMap::new(),
            command_handlers: HashMap::new(),

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

            command_definitions: self.command_definitions,
            command_handlers: self.command_handlers,

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
    pub fn command(mut self, definition: Command, handlers: Vec<(&str, CommandFunc<T>)>) -> Self {
        self.command_definitions
            .insert(definition.name.clone(), definition);

        for (name, func) in handlers {
            self.command_handlers.insert(
                String::from(name),
                CommandHandler {
                    module: self.name.clone(),
                    name: String::from(name),
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
