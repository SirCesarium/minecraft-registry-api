use crate::{error::ApiError, neoforge::{NeoForgeClient, models::InstallerQuery}};

impl NeoForgeClient {
    /// Downloads a `NeoForge` installer JAR for the given version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_installer(&self, params: InstallerQuery<'_>) -> Result<Vec<u8>, ApiError> {
        let url = format!(
            "https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar",
            version = params.version,
        );
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::neoforge::models::InstallerQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_download_installer() {
        let client = NeoForgeClient::new(Client::new());
        let meta = client.get_metadata().await.unwrap();
        let jar = client.download_installer(InstallerQuery { version: &meta.versioning.latest }).await.unwrap();
        assert!(jar.len() > 100_000);
    }
}
