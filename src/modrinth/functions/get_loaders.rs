use crate::{error::ApiError, modrinth::{ModrinthClient, BASE, models::Loader}};

impl ModrinthClient {
    /// Fetches all available mod loaders.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_loaders(&self) -> Result<Vec<Loader>, ApiError> {
        let url = format!("{BASE}/tag/loader");
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
    async fn test_get_loaders() {
        let client = ModrinthClient::new(Client::new());
        let loaders = client.get_loaders().await.unwrap();
        assert!(!loaders.is_empty());
    }
}
