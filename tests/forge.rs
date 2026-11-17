#![allow(clippy::unwrap_used, clippy::expect_used)]

mod support;

use minecraft_registry_api::forge::ForgeClient;
use minecraft_registry_api::forge::models::InstallerQuery;
use support::{MockHook, assert_http_error, body_mock, error_mock};

const PROMOS: &str = include_str!("fixtures/forge/promos.json");

#[tokio::test]
async fn get_promos_builds_url_and_maps_response() {
    let mock = body_mock(PROMOS);
    let client = ForgeClient::with_hook(mock.clone());

    let promos = client.get_promos().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json"
    );
    assert_eq!(promos.promos["1.21.4-recommended"], "54.1.16");
    assert_eq!(promos.promos["1.21.4-latest"], "54.1.19");
}

#[tokio::test]
async fn download_installer_builds_url_and_returns_bytes() {
    let mock = MockHook::new();
    mock.enqueue_bytes(b"forge-installer");
    let client = ForgeClient::with_hook(mock.clone());

    let bytes = client
        .download_installer(InstallerQuery {
            mc_version: "1.21.4",
            forge_version: "54.1.16",
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://maven.minecraftforge.net/net/minecraftforge/forge/1.21.4-54.1.16/forge-1.21.4-54.1.16-installer.jar"
    );
    assert_eq!(bytes, b"forge-installer");
}

#[tokio::test]
async fn download_installer_to_reports_progress_and_streams_bytes() {
    let mock = MockHook::new();
    mock.enqueue_bytes(b"forge-installer");
    let client = ForgeClient::with_hook(mock.clone());
    let mut collected = Vec::new();

    let (content_length, downloaded) = client
        .download_installer_to(
            InstallerQuery {
                mc_version: "1.21.4",
                forge_version: "54.1.16",
            },
            |chunk| collected.extend_from_slice(chunk),
        )
        .await
        .unwrap();

    assert_eq!(content_length, Some(15));
    assert_eq!(downloaded, 15);
    assert_eq!(collected, b"forge-installer");
}

#[tokio::test]
async fn get_promos_reports_server_error() {
    let mock = error_mock(500);
    let client = ForgeClient::with_hook(mock);

    let error = client.get_promos().await.unwrap_err();

    assert_http_error(&error, 500);
}
