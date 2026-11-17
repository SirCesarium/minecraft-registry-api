use std::sync::Arc;

use reqwest::Client;

use crate::hook::HttpHook;

mod functions;
pub mod models;

pub(crate) const MAVEN: &str = "https://maven.neoforged.net/releases";
pub(crate) const METADATA_PATH: &str = "/net/neoforged/neoforge/maven-metadata.xml";

pub struct NeoForgeClient {
    hook: Arc<dyn HttpHook>,
}

impl NeoForgeClient {
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
