use crate::{
    error::ApiError,
    forge::{ForgeClient, MAVEN, models::InstallerQuery},
    http,
};

impl ForgeClient {
    /// Downloads a `Forge` installer JAR for the given MC and Forge version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails.
    pub async fn download_installer(
        &self,
        params: InstallerQuery<'_>,
    ) -> Result<Vec<u8>, ApiError> {
        let url = installer_url(params.mc_version, params.forge_version);

        http::get_bytes(self.hook.as_ref(), &url, &[]).await
    }

    /// Downloads a `Forge` installer JAR, calling `on_chunk` for each received chunk.
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
        let url = installer_url(params.mc_version, params.forge_version);

        http::get_stream(self.hook.as_ref(), &url, &[], on_chunk).await
    }
}

fn installer_url(mc_version: &str, forge_version: &str) -> String {
    format!(
        "{MAVEN}/net/minecraftforge/forge/{mc_version}-{forge_version}/forge-{mc_version}-{forge_version}-installer.jar"
    )
}
