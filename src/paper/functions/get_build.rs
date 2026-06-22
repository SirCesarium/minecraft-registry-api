use crate::{
    error::ApiError,
    paper::{
        BASE, PaperClient,
        models::{BuildDownload, BuildQuery, PaperBuild},
    },
};

impl PaperClient {
    /// Fetches a specific build from `PaperMC` Fill v3 API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_build(&self, params: BuildQuery<'_>) -> Result<PaperBuild, ApiError> {
        let url = format!(
            "{BASE}/projects/{}/versions/{}/builds/{}",
            params.project, params.version, params.build
        );
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Downloads a build artifact from `PaperMC` Fill v3 API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build(&self, params: BuildDownload<'_>) -> Result<Vec<u8>, ApiError> {
        let build_info = self
            .get_build(BuildQuery {
                project: params.project,
                version: params.version,
                build: params.build,
            })
            .await?;
        let file = build_info
            .downloads
            .get("server:default")
            .ok_or_else(|| ApiError::from("server:default download not found".to_string()))?;
        let resp = self.client.get(&file.url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
    }

    /// Downloads a build artifact, calling `on_chunk` for each received chunk.
    ///
    /// Returns `(content_length, total_bytes_downloaded)`.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build_to<F: FnMut(&[u8])>(
        &self,
        params: BuildDownload<'_>,
        mut on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let build_info = self
            .get_build(BuildQuery {
                project: params.project,
                version: params.version,
                build: params.build,
            })
            .await?;
        let file = build_info
            .downloads
            .get("server:default")
            .ok_or_else(|| ApiError::from("server:default download not found".to_string()))?;
        let mut resp = self.client.get(&file.url).send().await?.error_for_status()?;
        let total = resp.content_length();
        let mut downloaded = 0u64;
        while let Some(chunk) = resp.chunk().await? {
            on_chunk(&chunk);
            downloaded += chunk.len() as u64;
        }
        Ok((total, downloaded))
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
        let b = client
            .get_build(BuildQuery {
                project: "paper",
                version: "1.21.4",
                build: 232,
            })
            .await
            .unwrap();
        assert_eq!(b.id, 232);
    }
}
