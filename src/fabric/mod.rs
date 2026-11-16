use std::sync::Arc;

use reqwest::Client;

use crate::transport::Transport;

mod functions;
pub mod models;

pub(crate) const BASE: &str = "https://meta.fabricmc.net/v2";
pub(crate) const MAVEN: &str = "https://maven.fabricmc.net";

pub struct FabricClient {
    transport: Arc<dyn Transport>,
}

impl FabricClient {
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
