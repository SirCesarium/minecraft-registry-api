use reqwest::Client;

mod functions;
mod models;

pub(crate) const BASE: &str = "https://api.purpurmc.org/v2";

pub struct PurpurClient {
    client: Client,
}

impl PurpurClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
