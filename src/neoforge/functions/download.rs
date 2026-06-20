use crate::{
    error::ApiError,
    neoforge::{NeoForgeClient, models::InstallerQuery},
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
        let url = format!(
            "https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar",
            version = params.version,
        );
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
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
        mut on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let url = format!(
            "https://maven.neoforged.net/releases/net/neoforged/neoforge/{version}/neoforge-{version}-installer.jar",
            version = params.version,
        );
        let mut resp = self.client.get(&url).send().await?.error_for_status()?;
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
    use crate::neoforge::models::InstallerQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_download_installer() {
        let client = NeoForgeClient::new(Client::new());
        let meta = client.get_metadata().await.unwrap();
        let jar = client
            .download_installer(InstallerQuery {
                version: &meta.versioning.latest,
            })
            .await
            .unwrap();
        assert!(jar.len() > 100_000);
    }
}
