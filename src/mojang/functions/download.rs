use crate::{error::ApiError, mojang::{MojangClient, OBJECTS_BASE}};

impl MojangClient {
    /// Downloads a file by its hash (e.g. server jar, assets).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download(&self, hash: &str, file: &str) -> Result<Vec<u8>, ApiError> {
        let url = format!("{OBJECTS_BASE}/{hash}/{file}");
        let resp = self.client.get(&url).send().await?;

        let bytes = resp.bytes().await?.to_vec();

        Ok(bytes)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

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
