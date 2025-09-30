use std::sync::Arc;

use twilight_gateway::Event;
use twilight_http::Client;
use twilight_model::id::{Id, marker::ApplicationMarker};

use crate::Metadata;

#[derive(Clone, Debug)]
pub struct EventContext<T: Clone + Send + Sync> {
    pub meta: Metadata,
    pub application_id: Id<ApplicationMarker>,
    pub services: Arc<T>,
    pub client: Arc<Client>,

    pub event: Event,
}
