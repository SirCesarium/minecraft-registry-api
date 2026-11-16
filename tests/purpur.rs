#![allow(clippy::unwrap_used, clippy::expect_used)]

mod support;

use minecraft_registry_api::purpur::PurpurClient;
use minecraft_registry_api::purpur::models::{BuildDownload, VersionQuery};
use support::{MockTransport, assert_http_error, body_mock, error_mock};

const PROJECT: &str = include_str!("fixtures/purpur/project.json");
const VERSION: &str = include_str!("fixtures/purpur/version.json");
const BUILD: &str = include_str!("fixtures/purpur/build.json");

#[tokio::test]
async fn get_project_builds_url_and_maps_response() {
    let mock = body_mock(PROJECT);
    let client = PurpurClient::with_transport(mock.clone());

    let project = client.get_project().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.purpurmc.org/v2/purpur"
    );
    assert_eq!(project.project, "purpur");
    assert_eq!(project.metadata.current, "1.21.4");
    assert_eq!(project.versions, ["1.21.4", "1.21.3"]);
}

#[tokio::test]
async fn get_version_builds_url_and_maps_response() {
    let mock = body_mock(VERSION);
    let client = PurpurClient::with_transport(mock.clone());

    let version = client
        .get_version(VersionQuery { version: "1.21.4" })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.purpurmc.org/v2/purpur/1.21.4"
    );
    assert_eq!(version.builds.latest, "2416");
    assert_eq!(version.builds.all.len(), 2);
}

#[tokio::test]
async fn get_build_builds_url_and_maps_response() {
    let mock = body_mock(BUILD);
    let client = PurpurClient::with_transport(mock.clone());

    let build = client.get_build("1.21.4", "2416").await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.purpurmc.org/v2/purpur/1.21.4/2416"
    );
    assert_eq!(build.build, "2416");
    assert_eq!(
        build.metadata.unwrap().type_field.as_deref(),
        Some("DEFAULT")
    );
}

#[tokio::test]
async fn download_build_builds_url_and_returns_bytes() {
    let mock = MockTransport::new();
    mock.enqueue_bytes(b"purpur-jar");
    let client = PurpurClient::with_transport(mock.clone());

    let bytes = client
        .download_build(BuildDownload {
            version: "1.21.4",
            build: "2416",
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.purpurmc.org/v2/purpur/1.21.4/2416/download"
    );
    assert_eq!(bytes, b"purpur-jar");
}

#[tokio::test]
async fn download_build_to_reports_progress_and_streams_bytes() {
    let mock = MockTransport::new();
    mock.enqueue_bytes(b"purpur-jar");
    let client = PurpurClient::with_transport(mock.clone());
    let mut collected = Vec::new();

    let (content_length, downloaded) = client
        .download_build_to(
            BuildDownload {
                version: "1.21.4",
                build: "2416",
            },
            |chunk| collected.extend_from_slice(chunk),
        )
        .await
        .unwrap();

    assert_eq!(content_length, Some(10));
    assert_eq!(downloaded, 10);
    assert_eq!(collected, b"purpur-jar");
}

#[tokio::test]
async fn get_version_reports_missing_version() {
    let mock = error_mock(404);
    let client = PurpurClient::with_transport(mock);

    let error = client
        .get_version(VersionQuery { version: "999.999" })
        .await
        .unwrap_err();

    assert_http_error(&error, 404);
}
