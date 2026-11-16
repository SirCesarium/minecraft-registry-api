use std::{future::Future, pin::Pin};

use reqwest::Response;

use crate::error::ApiError;

/// Executes the HTTP requests issued by the registry clients.
///
/// [`reqwest::Client`] is the default implementation. A custom implementation
/// replaces the network layer, which is how the test suite runs without a
/// server and how callers can route traffic through their own stack.
pub trait Transport: Send + Sync {
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

impl Transport for reqwest::Client {
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
