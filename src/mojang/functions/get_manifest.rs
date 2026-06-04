use crate::{error::ApiError, mojang::{MojangClient, models::MojangRoot}};

const MANIFEST_PATH: &str = "/mc/game/version_manifest_v2.json";

impl MojangClient {
    /// Fetches the Minecraft version manifest.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_manifest(&self) -> Result<MojangRoot, ApiError> {
        let url = format!("{}{}", self.base_url, MANIFEST_PATH);
        let resp = self.client.get(&url).send().await?;

        let manifest = resp.json().await?;

        Ok(manifest)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_fetch_manifest() {
        let client = MojangClient::new(Client::new());
        let v = client.get_manifest().await.unwrap();

        assert!(!v.versions.is_empty());
        assert!(!v.latest.release.is_empty());
    }
}