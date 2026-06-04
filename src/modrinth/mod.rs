use reqwest::Client;

mod functions;
mod models;

pub(crate) const BASE: &str = "https://api.modrinth.com/v2";

pub struct ModrinthClient {
    client: Client,
}

impl ModrinthClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
