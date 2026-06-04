use reqwest::Client;

mod models;

use crate::{error::MojangError, mojang::models::MojangRoot};

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const OBJECTS_URL: &str = "https://piston-data.mojang.com/v1/objects";

pub struct MojangClient {
    client: Client,
}

impl MojangClient {
    #[must_use]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Fetches the Minecraft version manifest.
    ///
    /// # Errors
    ///
    /// Returns [`MojangError::Http`] if the request or parsing fails.
    pub async fn get_manifest(&self) -> Result<MojangRoot, MojangError> {
        let resp = self.client.get(MANIFEST_URL).send().await?;

        let manifest = resp.json().await?;

        Ok(manifest)
    }

    /// Downloads a file by its hash (e.g. server jar, assets).
    ///
    /// # Errors
    ///
    /// Returns [`MojangError::Http`] if the request fails.
    pub async fn download(&self, hash: &str, file: &str) -> Result<Vec<u8>, MojangError> {
        let url = format!("{OBJECTS_URL}/{hash}/{file}");
        let resp = self.client.get(&url).send().await?;

        let bytes = resp.bytes().await?.to_vec();

        Ok(bytes)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::MojangClient;
    use reqwest::Client;

    #[tokio::test]
    async fn test_fetch_manifest() {
        let client = MojangClient::new(Client::new());
        let v = client.get_manifest().await.unwrap();

        assert!(!v.versions.is_empty());
        assert!(!v.latest.release.is_empty());
    }

    #[tokio::test]
    async fn test_download_server_jar() {
        let client = MojangClient::new(Client::new());
        let manifest = client.get_manifest().await.unwrap();

        let latest = manifest
            .versions
            .iter()
            .find(|v| v.id == manifest.latest.release && v.type_field == "release")
            .expect("latest release not found in versions");

        let resp = Client::new().get(&latest.url).send().await.unwrap();
        let version: serde_json::Value = resp.json().await.unwrap();

        let server = version["downloads"]["server"].as_object().unwrap();
        let hash = server["sha1"].as_str().unwrap();
        let url = server["url"].as_str().unwrap();
        let file = url.rsplit('/').next().unwrap();

        let jar = client.download(hash, file).await.unwrap();
        assert!(jar.len() > 1_000_000);
    }
}
