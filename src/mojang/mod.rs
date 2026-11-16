use std::sync::Arc;

use reqwest::Client;

use crate::transport::Transport;

mod functions;
pub mod models;

pub(crate) const META_BASE: &str = "https://piston-meta.mojang.com";
pub(crate) const OBJECTS_BASE: &str = "https://piston-data.mojang.com/v1/objects";

pub struct MojangClient {
    transport: Arc<dyn Transport>,
}

impl MojangClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self::with_transport(Arc::new(client))
    }

    /// Builds a client that sends requests through `transport` instead of the network.
    #[must_use]
    pub fn with_transport(transport: Arc<dyn Transport>) -> Self {
        Self { transport }
    }
}
