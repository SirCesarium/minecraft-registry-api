use crate::{error::ApiError, neoforge::{NeoForgeClient, META_URL, models::Metadata}};
use quick_xml::de::from_reader;

impl NeoForgeClient {
    /// Fetches `NeoForge` maven metadata with all available versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails,
    /// or [`ApiError::Xml`] if XML parsing fails.
    pub async fn get_metadata(&self) -> Result<Metadata, ApiError> {
        let resp = self.client.get(META_URL).send().await?.error_for_status()?;
        let bytes = resp.bytes().await?;
        let meta: Metadata = from_reader(&bytes[..]).map_err(|e| ApiError::Xml(e.to_string()))?;
        Ok(meta)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_metadata() {
        let client = NeoForgeClient::new(Client::new());
        let meta = client.get_metadata().await.unwrap();
        assert!(!meta.versioning.versions.list.is_empty());
    }
}
