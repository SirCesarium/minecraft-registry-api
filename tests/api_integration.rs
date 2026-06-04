use reqwest::Client;
use minecraft_registry_api::mojang::MojangClient;
use minecraft_registry_api::modrinth::ModrinthClient;
#[cfg(feature = "fabric")]
use minecraft_registry_api::fabric::FabricClient;
#[cfg(feature = "paper")]
use minecraft_registry_api::paper::PaperClient;
#[cfg(feature = "purpur")]
use minecraft_registry_api::purpur::PurpurClient;
#[cfg(feature = "forge")]
use minecraft_registry_api::forge::ForgeClient;
#[cfg(feature = "neoforge")]
use minecraft_registry_api::neoforge::NeoForgeClient;

fn client() -> Client {
    Client::builder()
        .user_agent("minecraft-registry-api-test/0.1")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_mojang_invalid_hash() {
    let c = MojangClient::new(client());
    let result = c.download("0000000000000000000000000000000000000000", "server.jar").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_modrinth_invalid_project() {
    let c = ModrinthClient::new(client());
    let result = c.get_project("this-project-does-not-exist-xyz").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_modrinth_invalid_version() {
    let c = ModrinthClient::new(client());
    let result = c.get_version("00000000").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_modrinth_empty_search() {
    let c = ModrinthClient::new(client());
    let result = c.search("zzzznonexistentqueryxxxx", 5, 0, None).await.unwrap();
    assert_eq!(result.total_hits, 0);
    assert!(result.hits.is_empty());
}

#[cfg(feature = "paper")]
#[tokio::test]
async fn test_paper_invalid_project() {
    let c = PaperClient::new(client());
    let result = c.get_project("not-a-real-project").await;
    assert!(result.is_err());
}

#[cfg(feature = "paper")]
#[tokio::test]
async fn test_paper_invalid_build() {
    let c = PaperClient::new(client());
    let result = c.get_build("paper", "1.21.4", 999999).await;
    assert!(result.is_err());
}

#[cfg(feature = "purpur")]
#[tokio::test]
async fn test_purpur_invalid_version() {
    let c = PurpurClient::new(client());
    let result = c.get_version("999.999").await;
    assert!(result.is_err());
}

#[cfg(feature = "fabric")]
#[tokio::test]
async fn test_fabric_invalid_installer() {
    let c = FabricClient::new(client());
    let result = c.download_installer("0.0.0-nonexistent").await;
    assert!(result.is_err());
}

#[cfg(feature = "forge")]
#[tokio::test]
async fn test_forge_invalid_version() {
    let c = ForgeClient::new(client());
    let result = c.download_installer("0.0", "0.0.0").await;
    assert!(result.is_err());
}

#[cfg(feature = "neoforge")]
#[tokio::test]
async fn test_neoforge_invalid_version() {
    let c = NeoForgeClient::new(client());
    let result = c.download_installer("0.0.0-nonexistent").await;
    assert!(result.is_err());
}
