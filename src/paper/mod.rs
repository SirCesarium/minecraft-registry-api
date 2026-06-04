use reqwest::Client;

mod functions;
mod models;

pub(crate) const BASE: &str = "https://api.papermc.io/v2";

pub struct PaperClient {
    client: Client,
}

impl PaperClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
