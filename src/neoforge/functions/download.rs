use crate::{
    error::ApiError,
    http,
    neoforge::{MAVEN, NeoForgeClient, models::InstallerQuery},
};

impl NeoForgeClient {
    /// Downloads a `NeoForge` installer JAR for the given version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_installer(
        &self,
        params: InstallerQuery<'_>,
    ) -> Result<Vec<u8>, ApiError> {
        let url = installer_url(params.version);

        http::get_bytes(self.hook.as_ref(), &url, &[]).await
    }

    /// Downloads a `NeoForge` installer JAR, calling `on_chunk` for each received chunk.
    ///
    /// Returns `(content_length, total_bytes_downloaded)`.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_installer_to<F: FnMut(&[u8])>(
        &self,
        params: InstallerQuery<'_>,
        on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let url = installer_url(params.version);

        http::get_stream(self.hook.as_ref(), &url, &[], on_chunk).await
    }
}

fn installer_url(version: &str) -> String {
    format!("{MAVEN}/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar")
}
