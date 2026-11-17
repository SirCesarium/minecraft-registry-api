use crate::{
    error::ApiError,
    forge::{ForgeClient, PROMO_URL, models::ForgePromos},
    http,
};

impl ForgeClient {
    /// Fetches Forge promotions JSON with latest versions per MC version.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request or parsing fails.
    pub async fn get_promos(&self) -> Result<ForgePromos, ApiError> {
        http::get_json(self.hook.as_ref(), PROMO_URL, &[]).await
    }
}
