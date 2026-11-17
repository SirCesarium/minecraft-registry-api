use crate::{
    error::ApiError,
    neoforge::{MAVEN, METADATA_PATH, NeoForgeClient, models::Metadata},
};
use quick_xml::de::from_reader;

impl NeoForgeClient {
    /// Fetches `NeoForge` maven metadata with all available versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request fails,
    /// or [`ApiError::Xml`] if XML parsing fails.
    pub async fn get_metadata(&self) -> Result<Metadata, ApiError> {
        let url = format!("{MAVEN}{METADATA_PATH}");
        let response = self.hook.get(&url, &[]).await?.error_for_status()?;
        let bytes = response.bytes().await?;
        let meta: Metadata = from_reader(&bytes[..]).map_err(|e| ApiError::Xml(e.to_string()))?;

        Ok(meta)
    }
}
