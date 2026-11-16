use crate::{
    error::ApiError,
    http,
    mojang::{MojangClient, OBJECTS_BASE, models::DownloadSpec},
};

impl MojangClient {
    /// Downloads a file by its hash (e.g. server jar, assets).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download(&self, params: DownloadSpec<'_>) -> Result<Vec<u8>, ApiError> {
        let url = format!("{OBJECTS_BASE}/{}/{}", params.hash, params.file);
        http::get_bytes(self.transport.as_ref(), &url, &[]).await
    }

    /// Downloads a file by its hash, calling `on_chunk` for each received chunk.
    ///
    /// Returns `(content_length, total_bytes_downloaded)`.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_to<F: FnMut(&[u8])>(
        &self,
        params: DownloadSpec<'_>,
        on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let url = format!("{OBJECTS_BASE}/{}/{}", params.hash, params.file);
        http::get_stream(self.transport.as_ref(), &url, &[], on_chunk).await
    }
}
