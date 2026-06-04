use crate::{
    error::ApiError,
    modrinth::{models::Facet, ModrinthClient, BASE, models::SearchResult},
};
use serde::Serialize;

#[derive(Serialize)]
struct SearchParams<'a> {
    query: &'a str,
    limit: u32,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    facets: Option<String>,
}

impl ModrinthClient {
    /// Searches for projects on Modrinth.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn search(
        &self,
        query: &str,
        limit: u32,
        offset: u32,
        facets: Option<Vec<Vec<Facet>>>,
    ) -> Result<SearchResult, ApiError> {
        let url = format!("{BASE}/search");
        let facets = facets.map(|groups| {
            let strings: Vec<Vec<String>> = groups
                .into_iter()
                .map(|group| group.into_iter().map(|f| f.to_string()).collect())
                .collect();
            serde_json::to_string(&strings).unwrap_or_default()
        });
        let params = SearchParams { query, limit, offset, facets };
        let resp = self.client.get(&url).query(&params).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::modrinth::models::{Facet, ProjectType};
    use reqwest::Client;

    #[tokio::test]
    async fn test_search() {
        let client = ModrinthClient::new(Client::new());
        let result = client.search("fabric-api", 5, 0, None).await.unwrap();
        assert!(!result.hits.is_empty());
    }

    #[tokio::test]
    async fn test_search_modpacks() {
        let client = ModrinthClient::new(Client::new());
        let facets = vec![vec![Facet::ProjectType(ProjectType::Modpack)]];
        let result = client.search("", 5, 0, Some(facets)).await.unwrap();
        assert!(result.total_hits > 0);
        assert_eq!(result.hits[0].project_type, "modpack");
    }


}
