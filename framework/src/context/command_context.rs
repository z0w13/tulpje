use std::sync::Arc;

use twilight_http::{client::InteractionClient, response::marker::EmptyBody, Client};
use twilight_model::{
    application::interaction::application_command::{
        CommandData, CommandDataOption, CommandOptionValue,
    },
    channel::{message::MessageFlags, Message},
    gateway::payload::incoming::InteractionCreate,
    guild::Guild,
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::ApplicationMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use super::Context;
use crate::{Error, Metadata};

#[derive(Clone, Debug)]
pub struct CommandContext<T: Clone + Send + Sync> {
    pub meta: Metadata,
    pub application_id: Id<ApplicationMarker>,
    pub services: Arc<T>,
    pub client: Arc<Client>,

    pub event: InteractionCreate,
    pub command: CommandData,
    pub options: Vec<CommandDataOption>,
}

impl<T: Clone + Send + Sync> CommandContext<T> {
    pub fn from_context(
        meta: Metadata,
        ctx: Context<T>,
        event: InteractionCreate,
        command: CommandData,
    ) -> Self {
        Self {
            meta,
            application_id: ctx.application_id,
            client: ctx.client,
            services: ctx.services,

            command,
            event,
            options: Vec::new(),
        }
    }

    pub fn interaction(&self) -> InteractionClient<'_> {
        self.client.interaction(self.application_id)
    }

    pub fn client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }

    pub async fn guild(&self) -> Result<Option<Guild>, Error> {
        let Some(guild_id) = self.event.guild_id else {
            return Ok(None);
        };

        Ok(Some(self.client.guild(guild_id).await?.model().await?))
    }

    pub async fn response(
        &self,
        response: InteractionResponse,
    ) -> Result<twilight_http::Response<EmptyBody>, twilight_http::Error> {
        self.interaction()
            .create_response(self.event.id, &self.event.token, &response)
            .await
    }

    pub async fn update(
        &self,
        message: impl Into<String>,
    ) -> Result<twilight_http::Response<Message>, twilight_http::Error> {
        self.interaction()
            .update_response(&self.event.token)
            .content(Some(&message.into()))
            .await
    }

    pub async fn reply(
        &self,
        message: impl Into<String>,
    ) -> Result<twilight_http::Response<EmptyBody>, twilight_http::Error> {
        let response = InteractionResponseDataBuilder::new()
            .content(message)
            .build();

        self.response(InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(response),
        })
        .await
    }

    pub async fn defer(&self) -> Result<twilight_http::Response<EmptyBody>, twilight_http::Error> {
        self.response(InteractionResponse {
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
            data: None,
        })
        .await
    }
    pub async fn defer_ephemeral(
        &self,
    ) -> Result<twilight_http::Response<EmptyBody>, twilight_http::Error> {
        self.response(InteractionResponse {
            kind: InteractionResponseType::DeferredChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .flags(MessageFlags::EPHEMERAL)
                    .build(),
            ),
        })
        .await
    }

    pub fn get_arg_string_optional(&self, name: &str) -> Result<Option<String>, Error> {
        let Some(opt) = self.options.iter().find(|opt| opt.name == name) else {
            return Ok(None);
        };

        let CommandOptionValue::String(value) = &opt.value else {
            return Err(format!("option '{}' not a string option", name).into());
        };

        Ok(Some(value.clone()))
    }

    pub fn get_arg_string(&self, name: &str) -> Result<String, Error> {
        self.get_arg_string_optional(name)?
            .ok_or_else(|| format!("couldn't find command argument {}", name).into())
    }
}
