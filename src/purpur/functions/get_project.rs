use crate::{error::ApiError, purpur::{PurpurClient, BASE, models::PurpurProject}};

impl PurpurClient {
    /// Fetches Purpur project info and available versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_project(&self) -> Result<PurpurProject, ApiError> {
        let url = format!("{BASE}/purpur");
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
    async fn test_get_purpur() {
        let client = PurpurClient::new(Client::new());
        let p = client.get_project().await.unwrap();
        assert_eq!(p.project, "purpur");
    }
}
