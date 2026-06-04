use crate::{error::ApiError, fabric::{FabricClient, BASE, models::FabricInstallerVersion}};

impl FabricClient {
    /// Fetches available `Fabric` installer versions from meta.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_installer_versions(&self) -> Result<Vec<FabricInstallerVersion>, ApiError> {
        let url = format!("{BASE}/versions/installer");
        let resp = self.client.get(&url).send().await?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_installer_versions() {
        let client = FabricClient::new(Client::new());
        let versions = client.get_installer_versions().await.unwrap();
        assert!(!versions.is_empty());
    }
}
