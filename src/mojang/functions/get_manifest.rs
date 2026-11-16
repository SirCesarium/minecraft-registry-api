use crate::{
    error::ApiError,
    http,
    mojang::{META_BASE, MojangClient, models::MojangRoot},
};

const MANIFEST_PATH: &str = "/mc/game/version_manifest_v2.json";

impl MojangClient {
    /// Fetches the Minecraft version manifest.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_manifest(&self) -> Result<MojangRoot, ApiError> {
        let url = format!("{META_BASE}{MANIFEST_PATH}");
        http::get_json(self.transport.as_ref(), &url, &[]).await
    }
}
