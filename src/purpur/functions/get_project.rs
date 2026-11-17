use crate::{
    error::ApiError,
    http,
    purpur::{BASE, PurpurClient, models::PurpurProject},
};

impl PurpurClient {
    /// Fetches Purpur project info and available versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_project(&self) -> Result<PurpurProject, ApiError> {
        let url = format!("{BASE}/purpur");
        http::get_json(self.hook.as_ref(), &url, &[]).await
    }
}
