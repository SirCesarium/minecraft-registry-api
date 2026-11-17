#![allow(clippy::unwrap_used, clippy::expect_used)]

mod support;

use minecraft_registry_api::fabric::FabricClient;
use minecraft_registry_api::fabric::models::InstallerQuery;
use support::{MockHook, assert_http_error, body_mock, error_mock};

const GAME_VERSIONS: &str = include_str!("fixtures/fabric/game_versions.json");
const LOADERS: &str = include_str!("fixtures/fabric/loaders.json");
const INSTALLERS: &str = include_str!("fixtures/fabric/installers.json");

#[tokio::test]
async fn get_game_versions_builds_url_and_maps_list() {
    let mock = body_mock(GAME_VERSIONS);
    let client = FabricClient::with_hook(mock.clone());

    let versions = client.get_game_versions().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://meta.fabricmc.net/v2/versions/game"
    );
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].version, "1.21.4");
    assert!(versions[0].stable);
    assert!(!versions[1].stable);
}

#[tokio::test]
async fn get_loaders_builds_url_and_maps_list() {
    let mock = body_mock(LOADERS);
    let client = FabricClient::with_hook(mock.clone());

    let loaders = client.get_loaders().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://meta.fabricmc.net/v2/versions/loader"
    );
    assert_eq!(loaders[0].version, "0.16.9");
    assert_eq!(loaders[0].build, 123);
}

#[tokio::test]
async fn get_installer_versions_builds_url_and_maps_list() {
    let mock = body_mock(INSTALLERS);
    let client = FabricClient::with_hook(mock.clone());

    let installers = client.get_installer_versions().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://meta.fabricmc.net/v2/versions/installer"
    );
    assert_eq!(installers[0].version, "1.1.1");
    assert_eq!(
        installers[0].url,
        "https://maven.fabricmc.net/net/fabricmc/fabric-installer/1.1.1/fabric-installer-1.1.1.jar"
    );
}

#[tokio::test]
async fn download_installer_builds_url_and_returns_bytes() {
    let mock = MockHook::new();
    mock.enqueue_bytes(b"fabric-installer");
    let client = FabricClient::with_hook(mock.clone());

    let bytes = client
        .download_installer(InstallerQuery { version: "1.1.1" })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://maven.fabricmc.net/net/fabricmc/fabric-installer/1.1.1/fabric-installer-1.1.1.jar"
    );
    assert_eq!(bytes, b"fabric-installer");
}

#[tokio::test]
async fn download_installer_to_reports_progress_and_streams_bytes() {
    let mock = MockHook::new();
    mock.enqueue_bytes(b"fabric-installer");
    let client = FabricClient::with_hook(mock.clone());
    let mut collected = Vec::new();

    let (content_length, downloaded) = client
        .download_installer_to(InstallerQuery { version: "1.1.1" }, |chunk| {
            collected.extend_from_slice(chunk);
        })
        .await
        .unwrap();

    assert_eq!(content_length, Some(16));
    assert_eq!(downloaded, 16);
    assert_eq!(collected, b"fabric-installer");
}

#[tokio::test]
async fn download_installer_reports_missing_version() {
    let mock = error_mock(404);
    let client = FabricClient::with_hook(mock);

    let error = client
        .download_installer(InstallerQuery {
            version: "0.0.0-nonexistent",
        })
        .await
        .unwrap_err();

    assert_http_error(&error, 404);
}
