#![allow(clippy::unwrap_used, clippy::expect_used)]

mod support;

use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::paper::PaperClient;
use minecraft_registry_api::paper::models::{BuildDownload, BuildQuery, ProjectQuery};
use support::{MockTransport, assert_http_error, body_mock, error_mock};

const PROJECT: &str = include_str!("fixtures/paper/project.json");
const VERSION: &str = include_str!("fixtures/paper/version.json");
const BUILD: &str = include_str!("fixtures/paper/build.json");
const BUILDS: &str = include_str!("fixtures/paper/builds.json");
const BUILD_WITHOUT_ARTIFACT: &str = include_str!("fixtures/paper/build_without_artifact.json");

#[tokio::test]
async fn get_project_builds_url_and_maps_response() {
    let mock = body_mock(PROJECT);
    let client = PaperClient::with_transport(mock.clone());

    let project = client
        .get_project(ProjectQuery { project: "paper" })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://fill.papermc.io/v3/projects/paper"
    );
    assert_eq!(project.project.id, "paper");
    assert_eq!(project.versions["1.21"], ["1.21.4", "1.21.3"]);
}

#[tokio::test]
async fn get_version_builds_url_and_maps_response() {
    let mock = body_mock(VERSION);
    let client = PaperClient::with_transport(mock.clone());

    let version = client.get_version("paper", "1.21.4").await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://fill.papermc.io/v3/projects/paper/versions/1.21.4"
    );
    assert_eq!(version.version.id, "1.21.4");
    assert_eq!(version.builds, [232, 231, 230]);
}

#[tokio::test]
async fn get_builds_by_channel_sends_channel_filter() {
    let mock = body_mock(BUILDS);
    let client = PaperClient::with_transport(mock.clone());

    let builds = client
        .get_builds_by_channel("paper", "1.21.4", "STABLE")
        .await
        .unwrap();

    let request = mock.only_request();
    assert_eq!(
        request.url,
        "https://fill.papermc.io/v3/projects/paper/versions/1.21.4/builds"
    );
    assert_eq!(request.query_param("channel"), Some("STABLE"));
    assert_eq!(builds.len(), 2);
    assert!(builds.iter().all(|build| build.channel == "STABLE"));
}

#[tokio::test]
async fn get_build_builds_url_and_maps_response() {
    let mock = body_mock(BUILD);
    let client = PaperClient::with_transport(mock.clone());

    let build = client
        .get_build(BuildQuery {
            project: "paper",
            version: "1.21.4",
            build: 232,
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://fill.papermc.io/v3/projects/paper/versions/1.21.4/builds/232"
    );
    assert_eq!(build.id, 232);
    assert_eq!(build.channel, "STABLE");
    assert_eq!(build.commits[0].sha, "abc123");
    assert_eq!(build.downloads["server:default"].size, 52_892_581);
}

#[tokio::test]
async fn get_build_reports_missing_build() {
    let mock = error_mock(404);
    let client = PaperClient::with_transport(mock);

    let error = client
        .get_build(BuildQuery {
            project: "paper",
            version: "1.21.4",
            build: 999_999,
        })
        .await
        .unwrap_err();

    assert_http_error(&error, 404);
}

#[tokio::test]
async fn download_build_fetches_the_artifact_url_from_the_build() {
    let mock = MockTransport::new();
    mock.enqueue_json(BUILD);
    mock.enqueue_bytes(b"paper-jar");
    let client = PaperClient::with_transport(mock.clone());

    let bytes = client
        .download_build(BuildDownload {
            project: "paper",
            version: "1.21.4",
            build: 232,
        })
        .await
        .unwrap();

    let requests = mock.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(
        requests[0].url,
        "https://fill.papermc.io/v3/projects/paper/versions/1.21.4/builds/232"
    );
    assert_eq!(requests[1].url, "https://cdn.test/paper-1.21.4-232.jar");
    assert_eq!(bytes, b"paper-jar");
}

#[tokio::test]
async fn download_build_to_reports_progress_and_streams_bytes() {
    let mock = MockTransport::new();
    mock.enqueue_json(BUILD);
    mock.enqueue_bytes(b"paper-jar");
    let client = PaperClient::with_transport(mock.clone());
    let mut collected = Vec::new();

    let (content_length, downloaded) = client
        .download_build_to(
            BuildDownload {
                project: "paper",
                version: "1.21.4",
                build: 232,
            },
            |chunk| collected.extend_from_slice(chunk),
        )
        .await
        .unwrap();

    assert_eq!(content_length, Some(9));
    assert_eq!(downloaded, 9);
    assert_eq!(collected, b"paper-jar");
}

#[tokio::test]
async fn download_build_without_server_artifact_reports_message() {
    let mock = body_mock(BUILD_WITHOUT_ARTIFACT);
    let client = PaperClient::with_transport(mock.clone());

    let error = client
        .download_build(BuildDownload {
            project: "paper",
            version: "1.21.4",
            build: 232,
        })
        .await
        .unwrap_err();

    assert!(matches!(error, ApiError::Msg(_)));
    assert_eq!(mock.requests().len(), 1);
}
