use std::sync::Arc;

use reqwest::Client;

use crate::transport::Transport;

mod functions;
pub mod models;

pub(crate) const PROMO_URL: &str =
    "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
pub(crate) const MAVEN: &str = "https://maven.minecraftforge.net";

pub struct ForgeClient {
    transport: Arc<dyn Transport>,
}

impl ForgeClient {
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
