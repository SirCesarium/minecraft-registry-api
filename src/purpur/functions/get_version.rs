use crate::{
    error::ApiError,
    http,
    purpur::{
        BASE, PurpurClient,
        models::{BuildDownload, PurpurBuildInfo, PurpurVersion, VersionQuery},
    },
};

impl PurpurClient {
    /// Fetches version info with build list from Purpur API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version(&self, params: VersionQuery<'_>) -> Result<PurpurVersion, ApiError> {
        let url = format!("{BASE}/purpur/{}", params.version);
        http::get_json(self.transport.as_ref(), &url, &[]).await
    }

    /// Fetches info for a specific Purpur build, including metadata (e.g. channel type).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_build(&self, version: &str, build: &str) -> Result<PurpurBuildInfo, ApiError> {
        let url = format!("{BASE}/purpur/{version}/{build}");
        http::get_json(self.transport.as_ref(), &url, &[]).await
    }

    /// Downloads a specific build of Purpur.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build(&self, params: BuildDownload<'_>) -> Result<Vec<u8>, ApiError> {
        let url = format!("{BASE}/purpur/{}/{}/download", params.version, params.build);
        http::get_bytes(self.transport.as_ref(), &url, &[]).await
    }

    /// Downloads a specific build of Purpur, calling `on_chunk` for each received chunk.
    ///
    /// Returns `(content_length, total_bytes_downloaded)`.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build_to<F: FnMut(&[u8])>(
        &self,
        params: BuildDownload<'_>,
        on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let url = format!("{BASE}/purpur/{}/{}/download", params.version, params.build);
        http::get_stream(self.transport.as_ref(), &url, &[], on_chunk).await
    }
}
