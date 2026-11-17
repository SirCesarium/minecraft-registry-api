use serde::de::DeserializeOwned;

use crate::{error::ApiError, hook::HttpHook};

pub(crate) async fn get_json<T: DeserializeOwned>(
    hook: &dyn HttpHook,
    url: &str,
    query: &[(&str, String)],
) -> Result<T, ApiError> {
    let response = hook.get(url, query).await?.error_for_status()?;
    Ok(response.json().await?)
}

pub(crate) async fn get_bytes(
    hook: &dyn HttpHook,
    url: &str,
    query: &[(&str, String)],
) -> Result<Vec<u8>, ApiError> {
    let response = hook.get(url, query).await?.error_for_status()?;
    Ok(response.bytes().await?.to_vec())
}

pub(crate) async fn get_stream<F: FnMut(&[u8])>(
    hook: &dyn HttpHook,
    url: &str,
    query: &[(&str, String)],
    mut on_chunk: F,
) -> Result<(Option<u64>, u64), ApiError> {
    let mut response = hook.get(url, query).await?.error_for_status()?;
    let content_length = response.content_length();
    let mut downloaded = 0u64;

    while let Some(chunk) = response.chunk().await? {
        on_chunk(&chunk);
        downloaded += chunk.len() as u64;
    }

    Ok((content_length, downloaded))
}
