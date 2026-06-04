use crate::{error::ApiError, modrinth::{ModrinthClient, BASE, models::SearchResult}};
use serde::Serialize;

#[derive(Serialize)]
struct SearchParams<'a> {
    query: &'a str,
    limit: u32,
    offset: u32,
}

impl ModrinthClient {
    /// Searches for projects on Modrinth.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn search(&self, query: &str, limit: u32, offset: u32) -> Result<SearchResult, ApiError> {
        let url = format!("{BASE}/search");
        let params = SearchParams { query, limit, offset };
        let resp = self.client.get(&url).query(&params).send().await?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_search() {
        let client = ModrinthClient::new(Client::new());
        let result = client.search("fabric-api", 5, 0).await.unwrap();
        assert!(!result.hits.is_empty());
    }
}
