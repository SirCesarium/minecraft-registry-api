use crate::{error::ApiError, modrinth::{ModrinthClient, BASE, models::Project}};

impl ModrinthClient {
    /// Fetches a project by slug or ID.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_project(&self, slug: &str) -> Result<Project, ApiError> {
        let url = format!("{BASE}/project/{slug}");
        let resp = self.client.get(&url).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_get_project() {
        let client = ModrinthClient::new(Client::new());
        let project = client.get_project("fabric-api").await.unwrap();
        assert_eq!(project.slug, "fabric-api");
    }
}
