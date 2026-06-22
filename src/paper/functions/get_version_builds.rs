use crate::{
    error::ApiError,
    paper::{
        BASE, PaperClient,
        models::PaperVersion,
    },
};

impl PaperClient {
    /// Fetches version info and available builds from `PaperMC` Fill v3 API.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_version(&self, project: &str, version: &str) -> Result<PaperVersion, ApiError> {
        let url = format!("{BASE}/projects/{project}/versions/{version}");
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
    async fn test_get_version() {
        let client = PaperClient::new(Client::new());
        let v = client.get_version("paper", "1.21.4").await.unwrap();
        assert!(!v.builds.is_empty());
    }
}
