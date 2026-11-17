use std::{future::Future, pin::Pin};

use reqwest::Response;

use crate::error::ApiError;

/// Intercepts the HTTP call made by a registry client.
///
/// [`reqwest::Client`] is the production implementation. The hook exists so
/// tests can return canned [`reqwest::Response`] values without a network or a
/// server, which is why implementations build on `reqwest`.
pub trait HttpHook: Send + Sync {
    /// Sends a GET request to `url` with the given query parameters.
    ///
    /// # Errors
    ///
    /// Returns [`ApiError::Http`] if the request cannot be built or executed.
    fn get<'a>(
        &'a self,
        url: &'a str,
        query: &'a [(&'a str, String)],
    ) -> Pin<Box<dyn Future<Output = Result<Response, ApiError>> + Send + 'a>>;
}

impl HttpHook for reqwest::Client {
    fn get<'a>(
        &'a self,
        url: &'a str,
        query: &'a [(&'a str, String)],
    ) -> Pin<Box<dyn Future<Output = Result<Response, ApiError>> + Send + 'a>> {
        Box::pin(async move {
            let request = self.get(url).query(query).build()?;
            Ok(self.execute(request).await?)
        })
    }
}
