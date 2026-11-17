use crate::{
    error::ApiError,
    http,
    modrinth::{
        BASE, ModrinthClient,
        models::{Version, VersionQuery},
    },
};

impl ModrinthClient {
    /// Fetches a single version by ID.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version(&self, params: VersionQuery<'_>) -> Result<Version, ApiError> {
        let url = format!("{BASE}/version/{}", params.version_id);
        http::get_json(self.hook.as_ref(), &url, &[]).await
    }
}
