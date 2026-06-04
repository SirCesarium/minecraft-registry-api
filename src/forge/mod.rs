use reqwest::Client;

mod functions;
mod models;

pub(crate) const PROMO_URL: &str = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";

pub struct ForgeClient {
    client: Client,
}

impl ForgeClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
