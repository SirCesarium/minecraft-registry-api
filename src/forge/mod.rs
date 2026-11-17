use std::sync::Arc;

use reqwest::Client;

use crate::hook::HttpHook;

mod functions;
pub mod models;

pub(crate) const PROMO_URL: &str =
    "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
pub(crate) const MAVEN: &str = "https://maven.minecraftforge.net";

pub struct ForgeClient {
    hook: Arc<dyn HttpHook>,
}

impl ForgeClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self::with_hook(Arc::new(client))
    }

    /// Builds a client with a custom HTTP hook. Intended for tests.
    #[must_use]
    pub fn with_hook(hook: Arc<dyn HttpHook>) -> Self {
        Self { hook }
    }
}
