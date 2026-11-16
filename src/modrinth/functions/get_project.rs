use crate::{
    error::ApiError,
    http,
    modrinth::{
        BASE, ModrinthClient,
        models::{Project, ProjectRef},
    },
};

impl ModrinthClient {
    /// Fetches a project by slug or ID.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_project(&self, params: ProjectRef<'_>) -> Result<Project, ApiError> {
        let url = format!("{BASE}/project/{}", params.slug);
        http::get_json(self.transport.as_ref(), &url, &[]).await
    }
}
