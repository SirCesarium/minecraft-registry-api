#![allow(clippy::unwrap_used, clippy::expect_used)]

mod support;

use minecraft_registry_api::mojang::MojangClient;
use minecraft_registry_api::mojang::models::DownloadSpec;
use support::{MockTransport, assert_http_error, body_mock, error_mock};

const MANIFEST: &str = include_str!("fixtures/mojang/manifest.json");
const HASH: &str = "0000000000000000000000000000000000000000";

#[tokio::test]
async fn get_manifest_builds_url_and_maps_response() {
    let mock = body_mock(MANIFEST);
    let client = MojangClient::with_transport(mock.clone());

    let manifest = client.get_manifest().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
    );
    assert_eq!(manifest.latest.release, "1.21.4");
    assert_eq!(manifest.versions.len(), 2);
    assert_eq!(manifest.versions[0].type_field, "release");
    assert_eq!(manifest.versions[0].compliance_level, 1);
}

#[tokio::test]
async fn get_manifest_reports_server_error() {
    let mock = error_mock(500);
    let client = MojangClient::with_transport(mock);

    let error = client.get_manifest().await.unwrap_err();

    assert_http_error(&error, 500);
}

#[tokio::test]
async fn download_builds_url_and_returns_bytes() {
    let mock = MockTransport::new();
    mock.enqueue_bytes(b"jar-bytes");
    let client = MojangClient::with_transport(mock.clone());

    let bytes = client
        .download(DownloadSpec {
            hash: HASH,
            file: "server.jar",
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        format!("https://piston-data.mojang.com/v1/objects/{HASH}/server.jar")
    );
    assert_eq!(bytes, b"jar-bytes");
}

#[tokio::test]
async fn download_to_reports_progress_and_streams_bytes() {
    let mock = MockTransport::new();
    mock.enqueue_bytes(b"streamed-jar");
    let client = MojangClient::with_transport(mock.clone());
    let mut collected = Vec::new();

    let (content_length, downloaded) = client
        .download_to(
            DownloadSpec {
                hash: HASH,
                file: "server.jar",
            },
            |chunk| collected.extend_from_slice(chunk),
        )
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        format!("https://piston-data.mojang.com/v1/objects/{HASH}/server.jar")
    );
    assert_eq!(content_length, Some(12));
    assert_eq!(downloaded, 12);
    assert_eq!(collected, b"streamed-jar");
}

#[tokio::test]
async fn download_to_reports_missing_object() {
    let mock = error_mock(404);
    let client = MojangClient::with_transport(mock);

    let error = client
        .download_to(
            DownloadSpec {
                hash: HASH,
                file: "server.jar",
            },
            |_| {},
        )
        .await
        .unwrap_err();

    assert_http_error(&error, 404);
}
