use crate::{error::ApiError, purpur::{PurpurClient, BASE, models::PurpurVersion}};

impl PurpurClient {
    /// Fetches version info with build list from Purpur API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version(&self, version: &str) -> Result<PurpurVersion, ApiError> {
        let url = format!("{BASE}/purpur/{version}");
        let resp = self.client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    /// Downloads a specific build of Purpur.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build(&self, version: &str, build: &str) -> Result<Vec<u8>, ApiError> {
        let url = format!("{BASE}/purpur/{version}/{build}/download");
        let resp = self.client.get(&url).send().await?;
        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_purpur_version() {
        let client = PurpurClient::new(Client::new());
        let v = client.get_version("1.21.4").await.unwrap();
        assert_eq!(v.version, "1.21.4");
    }
}
