use crate::{error::ApiError, forge::{ForgeClient, models::InstallerQuery}};

impl ForgeClient {
    /// Downloads a `Forge` installer JAR for the given MC and Forge version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_installer(&self, params: InstallerQuery<'_>) -> Result<Vec<u8>, ApiError> {
        let url = format!(
            "https://maven.minecraftforge.net/net/minecraftforge/forge/{mc_version}-{forge_version}/forge-{mc_version}-{forge_version}-installer.jar",
            mc_version = params.mc_version,
            forge_version = params.forge_version,
        );
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::forge::models::InstallerQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_download_installer() {
        let client = ForgeClient::new(Client::new());
        // Use promos to get a known version
        let promos = client.get_promos().await.unwrap();
        let latest = promos.promos.get("1.21.4-recommended")
            .or_else(|| promos.promos.get("1.21.4-latest"))
            .cloned()
            .unwrap_or_else(|| "54.1.16".to_string());
        let jar = client.download_installer(InstallerQuery { mc_version: "1.21.4", forge_version: &latest }).await.unwrap();
        assert!(jar.len() > 100_000);
    }
}
