use crate::{
    error::ApiError,
    http,
    paper::{
        BASE, PaperClient,
        models::{PaperProject, ProjectQuery},
    },
};

impl PaperClient {
    /// Fetches project information from `PaperMC` Fill v3 API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_project(&self, params: ProjectQuery<'_>) -> Result<PaperProject, ApiError> {
        let url = format!("{BASE}/projects/{}", params.project);
        http::get_json(self.hook.as_ref(), &url, &[]).await
    }
}
