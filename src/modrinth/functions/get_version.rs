use crate::{error::ApiError, modrinth::{ModrinthClient, BASE, models::Version}};

impl ModrinthClient {
    /// Fetches a single version by ID.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version(&self, version_id: &str) -> Result<Version, ApiError> {
        let url = format!("{BASE}/version/{version_id}");
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
    async fn test_get_version() {
        let client = ModrinthClient::new(Client::new());
        // grab first version from fabric-api as a known valid ID
        let versions = client.get_versions("fabric-api").await.unwrap();
        let first = versions.first().unwrap();
        let version = client.get_version(&first.id).await.unwrap();
        assert_eq!(version.id, first.id);
    }
}
