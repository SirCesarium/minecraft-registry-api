use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("{0}")]
    Msg(String),
    #[error("XML parsing failed: {0}")]
    Xml(String),
}

impl From<String> for ApiError {
    fn from(s: String) -> Self {
        Self::Msg(s)
    }
}
