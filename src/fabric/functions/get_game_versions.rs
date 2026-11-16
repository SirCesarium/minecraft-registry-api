use crate::{
    error::ApiError,
    fabric::{BASE, FabricClient, models::FabricGameVersion},
    http,
};

impl FabricClient {
    /// Fetches available Minecraft game versions from Fabric meta.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_game_versions(&self) -> Result<Vec<FabricGameVersion>, ApiError> {
        let url = format!("{BASE}/versions/game");
        http::get_json(self.transport.as_ref(), &url, &[]).await
    }
}
