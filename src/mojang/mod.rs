use std::sync::Arc;

use reqwest::Client;

use crate::hook::HttpHook;

mod functions;
pub mod models;

pub(crate) const META_BASE: &str = "https://piston-meta.mojang.com";
pub(crate) const OBJECTS_BASE: &str = "https://piston-data.mojang.com/v1/objects";

pub struct MojangClient {
    hook: Arc<dyn HttpHook>,
}

impl MojangClient {
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
