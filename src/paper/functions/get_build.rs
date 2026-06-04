use crate::{error::ApiError, paper::{PaperClient, BASE, models::PaperBuild}};

impl PaperClient {
    /// Fetches a specific build from `PaperMC` API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_build(&self, project: &str, version: &str, build: i64) -> Result<PaperBuild, ApiError> {
        let url = format!("{BASE}/projects/{project}/versions/{version}/builds/{build}");
        let resp = self.client.get(&url).send().await?;
        Ok(resp.json().await?)
    }

    /// Downloads a build artifact from `PaperMC` API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build(&self, project: &str, version: &str, build: i64) -> Result<Vec<u8>, ApiError> {
        let build_info = self.get_build(project, version, build).await?;
        let file = &build_info.downloads.application.name;
        let url = format!("{BASE}/projects/{project}/versions/{version}/builds/{build}/downloads/{file}");
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
    async fn test_get_build() {
        let client = PaperClient::new(Client::new());
        let b = client.get_build("paper", "1.21.4", 232).await.unwrap();
        assert_eq!(b.build, 232);
    }
}
