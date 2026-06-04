use reqwest::Client;

mod functions;
mod models;

const META_BASE: &str = "https://piston-meta.mojang.com";
const OBJECTS_BASE: &str = "https://piston-data.mojang.com/v1/objects";

pub struct MojangClient {
    client: Client,
    base_url: String,
}

impl MojangClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self {
            client,
            base_url: META_BASE.to_owned(),
        }
    }
}
