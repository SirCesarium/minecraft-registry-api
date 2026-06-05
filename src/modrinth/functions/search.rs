use crate::{
    error::ApiError,
    modrinth::{ModrinthClient, BASE, models::{SearchResult, SearchQuery}},
};
use serde::Serialize;

impl ModrinthClient {
    /// Searches for projects on Modrinth.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn search(
        &self,
        params: SearchQuery<'_>,
    ) -> Result<SearchResult, ApiError> {
        #[derive(Serialize)]
        struct QP<'a> {
            query: &'a str,
            limit: u32,
            offset: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            facets: Option<String>,
        }

        let url = format!("{BASE}/search");
        let facets = params.facets.map(|groups| {
            let strings: Vec<Vec<String>> = groups
                .into_iter()
                .map(|group| group.into_iter().map(|f| f.to_string()).collect())
                .collect();
            serde_json::to_string(&strings).unwrap_or_default()
        });
        let qp = QP {
            query: params.query,
            limit: params.limit,
            offset: params.offset,
            facets,
        };
        let resp = self.client.get(&url).query(&qp).send().await?.error_for_status()?;
        Ok(resp.json().await?)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::modrinth::models::{Facet, ProjectType, SearchQuery};
    use reqwest::Client;

    #[tokio::test]
    async fn test_search() {
        let client = ModrinthClient::new(Client::new());
        let result = client.search(SearchQuery { query: "fabric-api", limit: 5, offset: 0, facets: None }).await.unwrap();
        assert!(!result.hits.is_empty());
    }

    #[tokio::test]
    async fn test_search_modpacks() {
        let client = ModrinthClient::new(Client::new());
        let facets = vec![vec![Facet::ProjectType(ProjectType::Modpack)]];
        let result = client.search(SearchQuery { query: "", limit: 5, offset: 0, facets: Some(facets) }).await.unwrap();
        assert!(result.total_hits > 0);
        assert_eq!(result.hits[0].project_type, "modpack");
    }


}
