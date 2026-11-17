use std::sync::Arc;

use reqwest::Client;

use crate::hook::HttpHook;

mod functions;
pub mod models;

pub(crate) const BASE: &str = "https://meta.fabricmc.net/v2";
pub(crate) const MAVEN: &str = "https://maven.fabricmc.net";

pub struct FabricClient {
    hook: Arc<dyn HttpHook>,
}

impl FabricClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self::with_hook(Arc::new(client))
    }

    /// Builds a client with a custom HTTP hook. Intended for tests.
    #[doc(hidden)]
    #[must_use]
    pub fn with_hook(hook: Arc<dyn HttpHook>) -> Self {
        Self { hook }
    }
}
