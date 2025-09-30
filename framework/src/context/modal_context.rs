use std::sync::Arc;

use twilight_http::Client;
use twilight_model::{
    application::interaction::modal::ModalInteractionData,
    gateway::payload::incoming::InteractionCreate,
    id::{Id, marker::ApplicationMarker},
};

use super::Context;
use crate::Metadata;

#[derive(Clone, Debug)]
pub struct ModalContext<T: Clone + Send + Sync> {
    pub meta: Metadata,
    pub application_id: Id<ApplicationMarker>,
    pub services: Arc<T>,
    pub client: Arc<Client>,

    pub event: InteractionCreate,
    pub data: ModalInteractionData,
}

impl<T: Clone + Send + Sync> ModalContext<T> {
    pub fn from_context(
        ctx: Context<T>,
        meta: Metadata,
        event: InteractionCreate,
        data: ModalInteractionData,
    ) -> Self {
        Self {
            application_id: ctx.application_id,
            client: ctx.client,
            services: ctx.services,

            meta,
            data,
            event,
        }
    }
}
