use crate::{error::ApiError, fabric::FabricClient};

impl FabricClient {
    /// Downloads a `Fabric` installer JAR for the given version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_installer(&self, version: &str) -> Result<Vec<u8>, ApiError> {
        let url = format!("https://maven.fabricmc.net/net/fabricmc/fabric-installer/{version}/fabric-installer-{version}.jar");
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_download_installer() {
        let client = FabricClient::new(Client::new());
        let jar = client.download_installer("1.1.1").await.unwrap();
        assert!(jar.len() > 100_000);
    }
}
