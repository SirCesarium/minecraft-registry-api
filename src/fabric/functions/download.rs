use crate::{error::ApiError, fabric::{FabricClient, models::InstallerQuery}};

impl FabricClient {
    /// Downloads a `Fabric` installer JAR for the given version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_installer(&self, params: InstallerQuery<'_>) -> Result<Vec<u8>, ApiError> {
        let url = format!("https://maven.fabricmc.net/net/fabricmc/fabric-installer/{version}/fabric-installer-{version}.jar", version = params.version);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::fabric::models::InstallerQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_download_installer() {
        let client = FabricClient::new(Client::new());
        let jar = client.download_installer(InstallerQuery { version: "1.1.1" }).await.unwrap();
        assert!(jar.len() > 100_000);
    }
}
