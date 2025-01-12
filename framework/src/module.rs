use std::collections::{HashMap, HashSet};

use twilight_gateway::EventType;
use twilight_model::application::command::Command;

use crate::handler::{
    command_handler::CommandHandler, component_interaction_handler::ComponentInteractionHandler,
    event_handler::EventHandler, task_handler::TaskHandler,
};

pub mod builder;
pub mod command_builder;
pub mod registry;

#[derive(Clone)]
pub struct Module<T: Clone + Send + Sync> {
    pub(crate) name: String,
    pub(crate) guild_scoped: bool,

    pub(crate) commands: HashMap<String, CommandHandler<T>>,
    pub(crate) command_definitions: Vec<Command>,

    pub(crate) components: HashMap<String, ComponentInteractionHandler<T>>,
    pub(crate) events: HashMap<EventType, HashSet<EventHandler<T>>>,
    pub(crate) tasks: HashMap<String, TaskHandler<T>>,
}
