use crate::{
    error::ApiError,
    http,
    modrinth::{
        BASE, ModrinthClient,
        models::{Version, VersionListQuery},
    },
};

impl ModrinthClient {
    /// Fetches all versions of a project.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_versions(
        &self,
        params: VersionListQuery<'_>,
    ) -> Result<Vec<Version>, ApiError> {
        let url = format!("{BASE}/project/{}/version", params.project_id);
        http::get_json(self.transport.as_ref(), &url, &[]).await
    }
}
