use reqwest::Client;

mod functions;
pub mod models;

pub(crate) const BASE: &str = "https://fill.papermc.io/v3";

pub struct PaperClient {
    client: Client,
}

impl PaperClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
