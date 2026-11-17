use std::sync::Arc;

use reqwest::Client;

use crate::hook::HttpHook;

mod functions;
pub mod models;

pub(crate) const BASE: &str = "https://api.modrinth.com/v2";

pub struct ModrinthClient {
    hook: Arc<dyn HttpHook>,
}

impl ModrinthClient {
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
