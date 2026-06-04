use crate::{error::ApiError, fabric::{FabricClient, BASE, models::FabricGameVersion}};

impl FabricClient {
    /// Fetches available Minecraft game versions from Fabric meta.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_game_versions(&self) -> Result<Vec<FabricGameVersion>, ApiError> {
        let url = format!("{BASE}/versions/game");
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
    async fn test_get_game_versions() {
        let client = FabricClient::new(Client::new());
        let versions = client.get_game_versions().await.unwrap();
        assert!(!versions.is_empty());
    }
}
