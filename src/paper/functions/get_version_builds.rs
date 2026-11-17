use crate::{
    error::ApiError,
    http,
    paper::{
        BASE, PaperClient,
        models::{PaperBuild, PaperVersion},
    },
};

impl PaperClient {
    /// Fetches version info and available builds from `PaperMC` Fill v3 API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version(
        &self,
        project: &str,
        version: &str,
    ) -> Result<PaperVersion, ApiError> {
        let url = format!("{BASE}/projects/{project}/versions/{version}");
        http::get_json(self.hook.as_ref(), &url, &[]).await
    }

    /// Fetches builds for a version filtered by channel (e.g. "STABLE", "ALPHA", "BETA").
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_builds_by_channel(
        &self,
        project: &str,
        version: &str,
        channel: &str,
    ) -> Result<Vec<PaperBuild>, ApiError> {
        let url = format!("{BASE}/projects/{project}/versions/{version}/builds");
        let query = [("channel", channel.to_owned())];

        http::get_json(self.hook.as_ref(), &url, &query).await
    }
}
