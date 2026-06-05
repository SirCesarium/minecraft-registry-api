use reqwest::Client;

mod functions;
pub mod models;

pub(crate) const BASE: &str = "https://meta.fabricmc.net/v2";

pub struct FabricClient {
    client: Client,
}

impl FabricClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
