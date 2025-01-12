// Heavily influenced and largely copied from
// https://github.com/twilight-rs/twilight/blob/main/twilight-util/src/builder/command.rs

use std::collections::HashMap;

use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandOptionType, CommandType},
        interaction::InteractionContextType,
    },
    guild::Permissions,
    id::{marker::GuildMarker, Id},
    oauth::ApplicationIntegrationType,
};

use crate::handler::command_handler::CommandFunc;

#[derive(Debug, Clone)]
pub struct CommandBuilder<T: Clone + Send + Sync> {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,

    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,

    pub kind: CommandType,
    pub guild_id: Option<Id<GuildMarker>>,
    pub default_member_permissions: Option<Permissions>,
    pub contexts: Option<Vec<InteractionContextType>>,
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,
    pub nsfw: Option<bool>,

    pub func: Option<CommandFunc<T>>,
    pub groups: Vec<SubCommandGroupBuilder<T>>,
    pub subcommands: Vec<SubCommandBuilder<T>>,
    pub options: Vec<CommandOption>,
}

impl<T: Clone + Send + Sync> CommandBuilder<T> {
    pub fn new(name: impl Into<String>, description: impl Into<String>, kind: CommandType) -> Self {
        Self {
            name: name.into(),
            name_localizations: None,

            description: description.into(),
            description_localizations: None,

            kind,
            guild_id: None,
            default_member_permissions: None,
            contexts: None,
            integration_types: None,
            nsfw: None,

            func: None,
            groups: Vec::new(),
            subcommands: Vec::new(),
            options: Vec::new(),
        }
    }

    #[must_use]
    pub fn handler(mut self, handler: CommandFunc<T>) -> Self {
        self.func = Some(handler);
        self
    }

    #[must_use]
    pub fn group(mut self, group: SubCommandGroupBuilder<T>) -> Self {
        self.groups.push(group);
        self
    }

    #[must_use]
    pub fn subcommand(mut self, command: SubCommandBuilder<T>) -> Self {
        self.subcommands.push(command);
        self
    }

    #[must_use]
    pub fn guild_id(mut self, guild_id: Id<GuildMarker>) -> Self {
        self.guild_id = Some(guild_id);
        self
    }

    #[must_use]
    pub fn default_member_permissions(mut self, default_member_permissions: Permissions) -> Self {
        self.default_member_permissions = Some(default_member_permissions);
        self
    }

    #[must_use]
    pub fn contexts(mut self, contexts: impl IntoIterator<Item = InteractionContextType>) -> Self {
        self.contexts = Some(contexts.into_iter().collect());
        self
    }

    #[must_use]
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self
    }

    #[must_use]
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    #[must_use]
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    #[must_use]
    pub fn integration_types(
        mut self,
        integration_types: impl IntoIterator<Item = ApplicationIntegrationType>,
    ) -> Self {
        self.integration_types = Some(integration_types.into_iter().collect());
        self
    }

    #[must_use]
    pub fn option(mut self, option: impl Into<CommandOption>) -> Self {
        self.options.push(option.into());
        self
    }

    pub fn build(self) -> Command {
        let group_options: Vec<CommandOption> = self.groups.into_iter().map(Into::into).collect();
        let subcommand_options: Vec<CommandOption> =
            self.subcommands.into_iter().map(Into::into).collect();

        #[expect(
            deprecated,
            reason = "dm_permission is deprecated but need to specify all fields"
        )]
        Command {
            name: self.name,
            name_localizations: self.name_localizations,

            description: self.description,
            description_localizations: self.description_localizations,

            guild_id: self.guild_id,
            default_member_permissions: self.default_member_permissions,
            contexts: self.contexts,
            integration_types: self.integration_types,
            nsfw: self.nsfw,

            options: [self.options, group_options, subcommand_options].concat(),
            kind: self.kind,

            application_id: None,
            id: None,
            version: Id::new(1),

            dm_permission: None,
        }
    }
}

impl<T: Clone + Send + Sync> From<CommandBuilder<T>> for Command {
    fn from(value: CommandBuilder<T>) -> Self {
        value.build()
    }
}

#[derive(Debug, Clone)]
pub struct SubCommandGroupBuilder<T: Clone + Send + Sync> {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,

    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,

    pub commands: Vec<SubCommandBuilder<T>>,
}

impl<T: Clone + Send + Sync> SubCommandGroupBuilder<T> {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_localizations: None,

            description: description.into(),
            description_localizations: None,

            commands: Vec::new(),
        }
    }

    #[must_use]
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    #[must_use]
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    #[must_use]
    pub fn subcommand(mut self, subcommand: SubCommandBuilder<T>) -> Self {
        self.commands.push(subcommand);
        self
    }

    pub fn build(self) -> CommandOption {
        CommandOption {
            name: self.name,
            name_localizations: self.name_localizations,

            description: self.description,
            description_localizations: self.description_localizations,

            kind: CommandOptionType::SubCommandGroup,
            options: Some(self.commands.into_iter().map(Into::into).collect()),

            autocomplete: None,
            channel_types: None,
            choices: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            required: None,
        }
    }
}

impl<T: Clone + Send + Sync> From<SubCommandGroupBuilder<T>> for CommandOption {
    fn from(value: SubCommandGroupBuilder<T>) -> Self {
        value.build()
    }
}

#[derive(Debug, Clone)]
pub struct SubCommandBuilder<T: Clone + Send + Sync> {
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,

    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,

    pub func: Option<CommandFunc<T>>,
    pub options: Vec<CommandOption>,
}

impl<T: Clone + Send + Sync> SubCommandBuilder<T> {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_localizations: None,

            description: description.into(),
            description_localizations: None,

            func: None,
            options: Vec::new(),
        }
    }

    #[must_use]
    pub fn handler(mut self, handler: CommandFunc<T>) -> Self {
        self.func = Some(handler);
        self
    }

    #[must_use]
    pub fn description_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.name_localizations = Some(
            localizations
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    #[must_use]
    pub fn name_localizations<K: Into<String>, V: Into<String>>(
        mut self,
        localizations: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        self.description_localizations = Some(
            localizations
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    #[must_use]
    pub fn option(mut self, option: impl Into<CommandOption>) -> Self {
        self.options.push(option.into());
        self
    }

    pub fn build(self) -> CommandOption {
        CommandOption {
            name: self.name,
            name_localizations: self.name_localizations,

            description: self.description,
            description_localizations: self.description_localizations,

            options: Some(self.options),
            kind: CommandOptionType::SubCommand,

            autocomplete: None,
            channel_types: None,
            choices: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            required: None,
        }
    }
}

impl<T: Clone + Send + Sync> From<SubCommandBuilder<T>> for CommandOption {
    fn from(value: SubCommandBuilder<T>) -> Self {
        value.build()
    }
}
