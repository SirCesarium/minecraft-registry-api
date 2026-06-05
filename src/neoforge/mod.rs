use reqwest::Client;

mod functions;
pub mod models;

pub(crate) const META_URL: &str = "https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml";

pub struct NeoForgeClient {
    client: Client,
}

impl NeoForgeClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}
