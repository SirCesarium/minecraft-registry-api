// Each integration test binary links this module and uses a different subset.
#![allow(dead_code)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use minecraft_registry_api::{error::ApiError, transport::Transport};
use reqwest::{Response, StatusCode};

pub(crate) struct MockTransport {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    responses: VecDeque<MockResponse>,
    requests: Vec<RecordedRequest>,
}

struct MockResponse {
    status: u16,
    body: Vec<u8>,
}

#[derive(Clone, Debug)]
pub(crate) struct RecordedRequest {
    pub(crate) url: String,
    pub(crate) query: Vec<(String, String)>,
}

impl RecordedRequest {
    pub(crate) fn query_param(&self, name: &str) -> Option<&str> {
        self.query
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }
}

impl MockTransport {
    pub(crate) fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Inner::default()),
        })
    }

    pub(crate) fn enqueue_json(&self, body: &str) {
        self.enqueue(200, body.as_bytes().to_vec());
    }

    pub(crate) fn enqueue_bytes(&self, body: impl Into<Vec<u8>>) {
        self.enqueue(200, body.into());
    }

    pub(crate) fn enqueue_error(&self, status: u16) {
        self.enqueue(status, Vec::new());
    }

    fn enqueue(&self, status: u16, body: Vec<u8>) {
        self.inner
            .lock()
            .unwrap()
            .responses
            .push_back(MockResponse { status, body });
    }

    pub(crate) fn requests(&self) -> Vec<RecordedRequest> {
        self.inner.lock().unwrap().requests.clone()
    }

    pub(crate) fn only_request(&self) -> RecordedRequest {
        let requests = self.requests();
        assert_eq!(
            requests.len(),
            1,
            "expected exactly one request, got {requests:#?}"
        );

        requests.into_iter().next().unwrap()
    }
}

impl Transport for MockTransport {
    fn get<'a>(
        &'a self,
        url: &'a str,
        query: &'a [(&'a str, String)],
    ) -> Pin<Box<dyn Future<Output = Result<Response, ApiError>> + Send + 'a>> {
        let response = {
            let mut inner = self.inner.lock().unwrap();
            inner.requests.push(RecordedRequest {
                url: url.to_owned(),
                query: query
                    .iter()
                    .map(|(key, value)| ((*key).to_owned(), value.clone()))
                    .collect(),
            });

            inner
                .responses
                .pop_front()
                .unwrap_or_else(|| panic!("no mock response queued for {url}"))
        };

        Box::pin(async move {
            let response = http::Response::builder()
                .status(response.status)
                .body(reqwest::Body::from(response.body))
                .unwrap();

            Ok(Response::from(response))
        })
    }
}

pub(crate) fn body_mock(body: &str) -> Arc<MockTransport> {
    let mock = MockTransport::new();
    mock.enqueue_json(body);

    mock
}

pub(crate) fn error_mock(status: u16) -> Arc<MockTransport> {
    let mock = MockTransport::new();
    mock.enqueue_error(status);

    mock
}

pub(crate) fn assert_http_error(error: &ApiError, expected: u16) {
    match error {
        ApiError::Http(error) => assert_eq!(
            error.status(),
            Some(StatusCode::from_u16(expected).unwrap())
        ),
        other => panic!("expected an HTTP error, got {other:?}"),
    }
}

pub(crate) fn assert_decode_error(error: &ApiError) {
    match error {
        ApiError::Http(error) => {
            assert!(error.is_decode(), "expected a decode error, got {error:?}");
        }
        other => panic!("expected an HTTP error, got {other:?}"),
    }
}
