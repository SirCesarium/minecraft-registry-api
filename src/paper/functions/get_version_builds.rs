use crate::{
    error::ApiError,
    paper::{
        BASE, PaperClient,
        models::{PaperBuild, PaperVersion},
    },
};

impl PaperClient {
    /// Fetches version info and available builds from `PaperMC` Fill v3 API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version(&self, project: &str, version: &str) -> Result<PaperVersion, ApiError> {
        let url = format!("{BASE}/projects/{project}/versions/{version}");
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Fetches builds for a version filtered by channel (e.g. "STABLE", "ALPHA", "BETA").
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_builds_by_channel(
        &self,
        project: &str,
        version: &str,
        channel: &str,
    ) -> Result<Vec<PaperBuild>, ApiError> {
        let url = format!("{BASE}/projects/{project}/versions/{version}/builds");
        let resp = self
            .client
            .get(&url)
            .query(&[("channel", channel)])
            .send()
            .await?
            .error_for_status()?;
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
        let client = PaperClient::new(Client::new());
        let v = client.get_version("paper", "1.21.4").await.unwrap();
        assert!(!v.builds.is_empty());
    }

    #[tokio::test]
    async fn test_get_stable_builds() {
        let client = PaperClient::new(Client::new());
        let builds = client.get_builds_by_channel("paper", "26.1.2", "STABLE").await.unwrap();
        assert!(!builds.is_empty());
        assert!(builds.iter().all(|b| b.channel == "STABLE"));
    }
}
