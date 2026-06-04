use crate::{error::ApiError, forge::{ForgeClient, PROMO_URL, models::ForgePromos}};

impl ForgeClient {
    /// Fetches Forge promotions JSON with latest versions per MC version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_promos(&self) -> Result<ForgePromos, ApiError> {
        let resp = self.client.get(PROMO_URL).send().await?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_promos() {
        let client = ForgeClient::new(Client::new());
        let promos = client.get_promos().await.unwrap();
        assert!(!promos.promos.is_empty());
    }
}
