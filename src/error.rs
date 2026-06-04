use thiserror::Error;

#[derive(Debug, Error)]
pub enum MojangError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
}
