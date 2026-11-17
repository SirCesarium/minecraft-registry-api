use crate::{
    error::ApiError,
    http,
    modrinth::{BASE, ModrinthClient, models::Loader},
};

impl ModrinthClient {
    /// Fetches all available mod loaders.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_loaders(&self) -> Result<Vec<Loader>, ApiError> {
        let url = format!("{BASE}/tag/loader");
        http::get_json(self.hook.as_ref(), &url, &[]).await
    }
}
