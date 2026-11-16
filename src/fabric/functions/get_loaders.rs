use crate::{
    error::ApiError,
    fabric::{BASE, FabricClient, models::FabricLoaderVersion},
    http,
};

impl FabricClient {
    /// Fetches available Fabric loader versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_loaders(&self) -> Result<Vec<FabricLoaderVersion>, ApiError> {
        let url = format!("{BASE}/versions/loader");
        http::get_json(self.transport.as_ref(), &url, &[]).await
    }
}
