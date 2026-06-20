use crate::{
    error::ApiError,
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
        let resp = self.client.get(&url).send().await?.error_for_status()?;

        let bytes = resp.bytes().await?.to_vec();

        Ok(bytes)
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
        mut on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let url = format!("{OBJECTS_BASE}/{}/{}", params.hash, params.file);
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
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_download_server_jar() {
        let client = MojangClient::new(Client::new());
        let manifest = client.get_manifest().await.unwrap();

        let latest = manifest
            .versions
            .iter()
            .find(|v| v.id == manifest.latest.release && v.type_field == "release")
            .expect("latest release not found in versions");

        let resp = Client::new().get(&latest.url).send().await.unwrap();
        let version: serde_json::Value = resp.json().await.unwrap();

        let server = version["downloads"]["server"].as_object().unwrap();
        let hash = server["sha1"].as_str().unwrap();
        let url = server["url"].as_str().unwrap();
        let file = url.rsplit('/').next().unwrap();

        let jar = client.download(DownloadSpec { hash, file }).await.unwrap();
        assert!(jar.len() > 1_000_000);
    }
}
