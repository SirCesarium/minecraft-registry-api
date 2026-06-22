use crate::{
    error::ApiError,
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
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Fetches info for a specific Purpur build, including metadata (e.g. channel type).
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_build(
        &self,
        version: &str,
        build: &str,
    ) -> Result<PurpurBuildInfo, ApiError> {
        let url = format!("{BASE}/purpur/{version}/{build}");
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }

    /// Downloads a specific build of Purpur.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn download_build(&self, params: BuildDownload<'_>) -> Result<Vec<u8>, ApiError> {
        let url = format!("{BASE}/purpur/{}/{}/download", params.version, params.build);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.bytes().await?.to_vec())
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
        mut on_chunk: F,
    ) -> Result<(Option<u64>, u64), ApiError> {
        let url = format!("{BASE}/purpur/{}/{}/download", params.version, params.build);
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
    use crate::purpur::models::VersionQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_purpur_version() {
        let client = PurpurClient::new(Client::new());
        let v = client
            .get_version(VersionQuery { version: "1.21.4" })
            .await
            .unwrap();
        assert_eq!(v.version, "1.21.4");
    }

    #[tokio::test]
    async fn test_get_purpur_build() {
        let client = PurpurClient::new(Client::new());
        let b = client.get_build("1.21.4", "2416").await.unwrap();
        assert_eq!(b.build, "2416");
        assert!(b.metadata.is_none() || b.metadata.unwrap().type_field.is_none());
    }
}
