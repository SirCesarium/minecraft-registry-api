use crate::{
    error::ApiError,
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
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::paper::models::ProjectQuery;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_paper() {
        let client = PaperClient::new(Client::new());
        let p = client
            .get_project(ProjectQuery { project: "paper" })
            .await
            .unwrap();
        assert_eq!(p.project.id, "paper");
    }
}
