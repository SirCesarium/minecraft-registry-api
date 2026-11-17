use crate::{
    error::ApiError,
    fabric::{BASE, FabricClient, models::FabricInstallerVersion},
    http,
};

impl FabricClient {
    /// Fetches available `Fabric` installer versions from meta.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_installer_versions(&self) -> Result<Vec<FabricInstallerVersion>, ApiError> {
        let url = format!("{BASE}/versions/installer");
        http::get_json(self.hook.as_ref(), &url, &[]).await
    }
}
