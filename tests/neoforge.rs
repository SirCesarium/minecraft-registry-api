#![allow(clippy::unwrap_used, clippy::expect_used)]

mod support;

use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::neoforge::NeoForgeClient;
use minecraft_registry_api::neoforge::models::InstallerQuery;
use support::{MockTransport, assert_http_error, body_mock, error_mock};

const METADATA: &str = include_str!("fixtures/neoforge/metadata.xml");

#[tokio::test]
async fn get_metadata_builds_url_and_parses_xml() {
    let mock = body_mock(METADATA);
    let client = NeoForgeClient::with_transport(mock.clone());

    let metadata = client.get_metadata().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml"
    );
    assert_eq!(metadata.group_id, "net.neoforged");
    assert_eq!(metadata.versioning.latest, "21.4.0-beta");
    assert_eq!(metadata.versioning.versions.list.len(), 2);
}

#[tokio::test]
async fn get_metadata_reports_invalid_xml() {
    let mock = body_mock("not xml at all");
    let client = NeoForgeClient::with_transport(mock);

    let error = client.get_metadata().await.unwrap_err();

    assert!(matches!(error, ApiError::Xml(_)), "got {error:?}");
}

#[tokio::test]
async fn get_metadata_reports_missing_metadata() {
    let mock = error_mock(404);
    let client = NeoForgeClient::with_transport(mock);

    let error = client.get_metadata().await.unwrap_err();

    assert_http_error(&error, 404);
}

#[tokio::test]
async fn download_installer_builds_url_and_returns_bytes() {
    let mock = MockTransport::new();
    mock.enqueue_bytes(b"neoforge-installer");
    let client = NeoForgeClient::with_transport(mock.clone());

    let bytes = client
        .download_installer(InstallerQuery {
            version: "21.4.0-beta",
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://maven.neoforged.net/releases/net/neoforged/neoforge/21.4.0-beta/neoforge-21.4.0-beta-installer.jar"
    );
    assert_eq!(bytes, b"neoforge-installer");
}

#[tokio::test]
async fn download_installer_to_reports_progress_and_streams_bytes() {
    let mock = MockTransport::new();
    mock.enqueue_bytes(b"neoforge-installer");
    let client = NeoForgeClient::with_transport(mock.clone());
    let mut collected = Vec::new();

    let (content_length, downloaded) = client
        .download_installer_to(
            InstallerQuery {
                version: "21.4.0-beta",
            },
            |chunk| {
                collected.extend_from_slice(chunk);
            },
        )
        .await
        .unwrap();

    assert_eq!(content_length, Some(18));
    assert_eq!(downloaded, 18);
    assert_eq!(collected, b"neoforge-installer");
}

#[tokio::test]
async fn download_installer_reports_missing_version() {
    let mock = error_mock(404);
    let client = NeoForgeClient::with_transport(mock);

    let error = client
        .download_installer(InstallerQuery {
            version: "0.0.0-nonexistent",
        })
        .await
        .unwrap_err();

    assert_http_error(&error, 404);
}
