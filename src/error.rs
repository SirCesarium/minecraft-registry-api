use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("XML parsing failed: {0}")]
    Xml(String),
}

impl From<String> for ApiError {
    fn from(s: String) -> Self {
        Self::Xml(s)
    }
}
