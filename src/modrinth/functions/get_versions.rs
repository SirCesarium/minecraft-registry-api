use crate::{error::ApiError, modrinth::{ModrinthClient, BASE, models::{Version, VersionListQuery}}};

impl ModrinthClient {
    /// Fetches all versions of a project.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_versions(&self, params: VersionListQuery<'_>) -> Result<Vec<Version>, ApiError> {
        let url = format!("{BASE}/project/{}/version", params.project_id);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::modrinth::models::VersionListQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_versions() {
        let client = ModrinthClient::new(Client::new());
        let versions = client.get_versions(VersionListQuery { project_id: "fabric-api" }).await.unwrap();
        assert!(!versions.is_empty());
    }
}
