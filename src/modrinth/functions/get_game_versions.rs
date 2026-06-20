use crate::{
    error::ApiError,
    modrinth::{BASE, ModrinthClient, models::GameVersion},
};

impl ModrinthClient {
    /// Fetches all available game versions.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_game_versions(&self) -> Result<Vec<GameVersion>, ApiError> {
        let url = format!("{BASE}/tag/game_version");
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
    async fn test_get_game_versions() {
        let client = ModrinthClient::new(Client::new());
        let versions = client.get_game_versions().await.unwrap();
        assert!(!versions.is_empty());
    }
}
