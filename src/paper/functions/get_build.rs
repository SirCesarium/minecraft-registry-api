use crate::{error::ApiError, paper::{PaperClient, BASE, models::{PaperBuild, BuildQuery, BuildDownload}}};

impl PaperClient {
    /// Fetches a specific build from `PaperMC` API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_build(&self, params: BuildQuery<'_>) -> Result<PaperBuild, ApiError> {
        let url = format!("{BASE}/projects/{}/versions/{}/builds/{}", params.project, params.version, params.build);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Downloads a build artifact from `PaperMC` API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build(&self, params: BuildDownload<'_>) -> Result<Vec<u8>, ApiError> {
        let build_info = self.get_build(BuildQuery { project: params.project, version: params.version, build: params.build }).await?;
        let file = &build_info.downloads.application.name;
        let url = format!("{BASE}/projects/{}/versions/{}/builds/{}/downloads/{}", params.project, params.version, params.build, file);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::paper::models::BuildQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_build() {
        let client = PaperClient::new(Client::new());
        let b = client.get_build(BuildQuery { project: "paper", version: "1.21.4", build: 232 }).await.unwrap();
        assert_eq!(b.build, 232);
    }
}
