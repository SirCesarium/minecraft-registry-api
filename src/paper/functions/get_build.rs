use crate::{
    error::ApiError,
    http,
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

        http::get_json(self.transport.as_ref(), &url, &[]).await
    }

    /// Downloads a build artifact from `PaperMC` Fill v3 API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails, or
    /// [`ApiError::Msg`] if the build has no `server:default` artifact.
    pub async fn download_build(&self, params: BuildDownload<'_>) -> Result<Vec<u8>, ApiError> {
        let url = self.artifact_url(&params).await?;

        http::get_bytes(self.transport.as_ref(), &url, &[]).await
    }

    /// Downloads a build artifact, calling `on_chunk` for each received chunk.
    ///
    /// Returns `(content_length, total_bytes_downloaded)`.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails, or
    /// [`ApiError::Msg`] if the build has no `server:default` artifact.
    pub async fn download_build_to<F: FnMut(&[u8])>(
        &self,
        params: BuildDownload<'_>,
        on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let url = self.artifact_url(&params).await?;

        http::get_stream(self.transport.as_ref(), &url, &[], on_chunk).await
    }

    async fn artifact_url(&self, params: &BuildDownload<'_>) -> Result<String, ApiError> {
        let build = self
            .get_build(BuildQuery {
                project: params.project,
                version: params.version,
                build: params.build,
            })
            .await?;

        build
            .downloads
            .get("server:default")
            .map(|file| file.url.clone())
            .ok_or_else(|| ApiError::from("server:default download not found".to_string()))
    }
}
