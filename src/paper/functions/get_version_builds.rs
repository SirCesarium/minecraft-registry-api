use crate::{
    error::ApiError,
    paper::{
        BASE, PaperClient,
        models::{PaperVersionBuilds, VersionBuildsQuery},
    },
};

impl PaperClient {
    /// Fetches all builds for a given Paper version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version_builds(
        &self,
        params: VersionBuildsQuery<'_>,
    ) -> Result<PaperVersionBuilds, ApiError> {
        let url = format!(
            "{BASE}/projects/{}/versions/{}/builds",
            params.project, params.version
        );
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::paper::models::VersionBuildsQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_version_builds() {
        let client = PaperClient::new(Client::new());
        let builds = client
            .get_version_builds(VersionBuildsQuery {
                project: "paper",
                version: "1.21.4",
            })
            .await
            .unwrap();
        assert!(!builds.builds.is_empty());
    }
}
