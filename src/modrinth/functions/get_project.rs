use crate::{error::ApiError, modrinth::{ModrinthClient, BASE, models::{Project, ProjectRef}}};

impl ModrinthClient {
    /// Fetches a project by slug or ID.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_project(&self, params: ProjectRef<'_>) -> Result<Project, ApiError> {
        let url = format!("{BASE}/project/{}", params.slug);
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::modrinth::models::ProjectRef;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_project() {
        let client = ModrinthClient::new(Client::new());
        let project = client.get_project(ProjectRef { slug: "fabric-api" }).await.unwrap();
        assert_eq!(project.slug, "fabric-api");
    }
}
