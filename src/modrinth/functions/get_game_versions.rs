use crate::{
    error::ApiError,
    http,
    modrinth::{BASE, ModrinthClient, models::GameVersion},
};

impl ModrinthClient {
    /// Fetches all available game versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_game_versions(&self) -> Result<Vec<GameVersion>, ApiError> {
        let url = format!("{BASE}/tag/game_version");
        http::get_json(self.hook.as_ref(), &url, &[]).await
    }
}
