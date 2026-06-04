use crate::{error::ApiError, paper::{PaperClient, BASE, models::PaperProject}};

impl PaperClient {
    /// Fetches project information from `PaperMC` API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_project(&self, project: &str) -> Result<PaperProject, ApiError> {
        let url = format!("{BASE}/projects/{project}");
        let resp = self.client.get(&url).send().await?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_paper() {
        let client = PaperClient::new(Client::new());
        let p = client.get_project("paper").await.unwrap();
        assert_eq!(p.project_id, "paper");
    }
}
