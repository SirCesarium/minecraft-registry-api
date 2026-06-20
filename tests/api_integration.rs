#![allow(clippy::unwrap_used)]

#[cfg(feature = "fabric")]
use minecraft_registry_api::fabric::FabricClient;
#[cfg(feature = "fabric")]
use minecraft_registry_api::fabric::models::InstallerQuery as FabricInstallerQuery;
#[cfg(feature = "forge")]
use minecraft_registry_api::forge::ForgeClient;
#[cfg(feature = "forge")]
use minecraft_registry_api::forge::models::InstallerQuery as ForgeInstallerQuery;
use minecraft_registry_api::modrinth::ModrinthClient;
use minecraft_registry_api::modrinth::models::{ProjectRef, SearchQuery, VersionQuery};
use minecraft_registry_api::mojang::MojangClient;
use minecraft_registry_api::mojang::models::DownloadSpec;
#[cfg(feature = "neoforge")]
use minecraft_registry_api::neoforge::NeoForgeClient;
#[cfg(feature = "neoforge")]
use minecraft_registry_api::neoforge::models::InstallerQuery as NeoForgeInstallerQuery;
#[cfg(feature = "paper")]
use minecraft_registry_api::paper::PaperClient;
#[cfg(feature = "paper")]
use minecraft_registry_api::paper::models::{BuildQuery, ProjectQuery};
#[cfg(feature = "purpur")]
use minecraft_registry_api::purpur::PurpurClient;
#[cfg(feature = "purpur")]
use minecraft_registry_api::purpur::models::VersionQuery as PurpurVersionQuery;
use reqwest::Client;

fn client() -> Client {
    Client::builder()
        .user_agent("minecraft-registry-api-test/0.1")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_mojang_invalid_hash() {
    let c = MojangClient::new(client());
    let result = c
        .download(DownloadSpec {
            hash: "0000000000000000000000000000000000000000",
            file: "server.jar",
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_modrinth_invalid_project() {
    let c = ModrinthClient::new(client());
    let result = c
        .get_project(ProjectRef {
            slug: "this-project-does-not-exist-xyz",
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_modrinth_invalid_version() {
    let c = ModrinthClient::new(client());
    let result = c
        .get_version(VersionQuery {
            version_id: "00000000",
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_modrinth_empty_search() {
    let c = ModrinthClient::new(client());
    let result = c
        .search(SearchQuery {
            query: "zzzznonexistentqueryxxxx",
            limit: 5,
            offset: 0,
            facets: None,
        })
        .await
        .unwrap();
    assert_eq!(result.total_hits, 0);
    assert!(result.hits.is_empty());
}

#[cfg(feature = "paper")]
#[tokio::test]
async fn test_paper_invalid_project() {
    let c = PaperClient::new(client());
    let result = c
        .get_project(ProjectQuery {
            project: "not-a-real-project",
        })
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "paper")]
#[tokio::test]
async fn test_paper_invalid_build() {
    let c = PaperClient::new(client());
    let result = c
        .get_build(BuildQuery {
            project: "paper",
            version: "1.21.4",
            build: 999_999,
        })
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "purpur")]
#[tokio::test]
async fn test_purpur_invalid_version() {
    let c = PurpurClient::new(client());
    let result = c
        .get_version(PurpurVersionQuery { version: "999.999" })
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "fabric")]
#[tokio::test]
async fn test_fabric_invalid_installer() {
    let c = FabricClient::new(client());
    let result = c
        .download_installer(FabricInstallerQuery {
            version: "0.0.0-nonexistent",
        })
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "forge")]
#[tokio::test]
async fn test_forge_invalid_version() {
    let c = ForgeClient::new(client());
    let result = c
        .download_installer(ForgeInstallerQuery {
            mc_version: "0.0",
            forge_version: "0.0.0",
        })
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "neoforge")]
#[tokio::test]
async fn test_neoforge_invalid_version() {
    let c = NeoForgeClient::new(client());
    let result = c
        .download_installer(NeoForgeInstallerQuery {
            version: "0.0.0-nonexistent",
        })
        .await;
    assert!(result.is_err());
}

// --- Streaming `_to` tests ---

#[tokio::test]
async fn test_mojang_download_to_invalid_hash() {
    let c = MojangClient::new(client());
    let (content_length, downloaded) = c
        .download_to(
            DownloadSpec {
                hash: "0000000000000000000000000000000000000000",
                file: "server.jar",
            },
            |_| {},
        )
        .await
        .unwrap_or((None, 0));
    assert_eq!(content_length, None);
    assert_eq!(downloaded, 0);
}

#[tokio::test]
async fn test_mojang_download_to_invalid_http() {
    let c = MojangClient::new(client());
    let result = c
        .download_to(
            DownloadSpec {
                hash: "0000000000000000000000000000000000000000",
                file: "server.jar",
            },
            |_| {},
        )
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "fabric")]
#[tokio::test]
async fn test_fabric_download_installer_to_invalid() {
    let c = FabricClient::new(client());
    let result = c
        .download_installer_to(
            FabricInstallerQuery {
                version: "0.0.0-nonexistent",
            },
            |_| {},
        )
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "forge")]
#[tokio::test]
async fn test_forge_download_installer_to_invalid() {
    let c = ForgeClient::new(client());
    let result = c
        .download_installer_to(
            ForgeInstallerQuery {
                mc_version: "0.0",
                forge_version: "0.0.0",
            },
            |_| {},
        )
        .await;
    assert!(result.is_err());
}

#[cfg(feature = "neoforge")]
#[tokio::test]
async fn test_neoforge_download_installer_to_invalid() {
    let c = NeoForgeClient::new(client());
    let result = c
        .download_installer_to(
            NeoForgeInstallerQuery {
                version: "0.0.0-nonexistent",
            },
            |_| {},
        )
        .await;
    assert!(result.is_err());
}
