use crate::{
    error::ApiError,
    fabric::{BASE, FabricClient, models::FabricLoaderVersion},
};

impl FabricClient {
    /// Fetches available Fabric loader versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_loaders(&self) -> Result<Vec<FabricLoaderVersion>, ApiError> {
        let url = format!("{BASE}/versions/loader");
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_loaders() {
        let client = FabricClient::new(Client::new());
        let loaders = client.get_loaders().await.unwrap();
        assert!(!loaders.is_empty());
    }
}
