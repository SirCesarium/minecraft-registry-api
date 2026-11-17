use crate::{
    error::ApiError,
    http,
    modrinth::{
        BASE, ModrinthClient,
        models::{SearchQuery, SearchResult},
    },
};

impl ModrinthClient {
    /// Searches for projects on Modrinth.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn search(&self, params: SearchQuery<'_>) -> Result<SearchResult, ApiError> {
        let mut query = vec![
            ("query", params.query.to_owned()),
            ("limit", params.limit.to_string()),
            ("offset", params.offset.to_string()),
        ];

        if let Some(groups) = params.facets {
            let strings: Vec<Vec<String>> = groups
                .into_iter()
                .map(|group| group.into_iter().map(|facet| facet.to_string()).collect())
                .collect();
            let facets = serde_json::to_string(&strings).unwrap_or_default();
            query.push(("facets", facets));
        }

        let url = format!("{BASE}/search");

        http::get_json(self.hook.as_ref(), &url, &query).await
    }
}
