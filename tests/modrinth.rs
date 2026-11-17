#![allow(clippy::unwrap_used, clippy::expect_used)]

mod support;

use minecraft_registry_api::modrinth::ModrinthClient;
use minecraft_registry_api::modrinth::models::{
    Facet, ProjectRef, ProjectType, SearchQuery, VersionListQuery, VersionQuery,
};
use support::{assert_decode_error, assert_http_error, body_mock, error_mock};

const PROJECT: &str = include_str!("fixtures/modrinth/project.json");
const SEARCH: &str = include_str!("fixtures/modrinth/search.json");
const VERSION: &str = include_str!("fixtures/modrinth/version.json");
const VERSIONS: &str = include_str!("fixtures/modrinth/versions.json");
const LOADERS: &str = include_str!("fixtures/modrinth/loaders.json");
const GAME_VERSIONS: &str = include_str!("fixtures/modrinth/game_versions.json");

#[tokio::test]
async fn get_project_builds_url_and_maps_response() {
    let mock = body_mock(PROJECT);
    let client = ModrinthClient::with_hook(mock.clone());

    let project = client
        .get_project(ProjectRef { slug: "fabric-api" })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.modrinth.com/v2/project/fabric-api"
    );
    assert_eq!(project.slug, "fabric-api");
    assert_eq!(project.id, "P7dR8mSH");
    assert_eq!(project.license.id, "Apache-2.0");
    assert_eq!(project.loaders, ["fabric", "quilt"]);
    assert_eq!(
        project.issues_url.as_deref(),
        Some("https://example.test/issues")
    );
    assert_eq!(project.wiki_url, None);
    assert_eq!(project.color, Some(1_234_567));
}

#[tokio::test]
async fn get_project_reports_missing_project() {
    let mock = error_mock(404);
    let client = ModrinthClient::with_hook(mock);

    let error = client
        .get_project(ProjectRef {
            slug: "missing-project",
        })
        .await
        .unwrap_err();

    assert_http_error(&error, 404);
}

#[tokio::test]
async fn get_project_reports_malformed_json() {
    let mock = body_mock(r#"{"slug": "#);
    let client = ModrinthClient::with_hook(mock);

    let error = client
        .get_project(ProjectRef { slug: "fabric-api" })
        .await
        .unwrap_err();

    assert_decode_error(&error);
}

#[tokio::test]
async fn search_sends_query_parameters() {
    let mock = body_mock(SEARCH);
    let client = ModrinthClient::with_hook(mock.clone());

    let result = client
        .search(SearchQuery {
            query: "sodium",
            limit: 10,
            offset: 20,
            facets: None,
        })
        .await
        .unwrap();

    let request = mock.only_request();
    assert_eq!(request.url, "https://api.modrinth.com/v2/search");
    assert_eq!(request.query_param("query"), Some("sodium"));
    assert_eq!(request.query_param("limit"), Some("10"));
    assert_eq!(request.query_param("offset"), Some("20"));
    assert_eq!(request.query_param("facets"), None);
    assert_eq!(result.total_hits, 1);
    assert_eq!(result.offset, 20);
    assert_eq!(result.hits[0].slug, "sodium");
    assert_eq!(result.hits[0].color, Some(3_949_738));
}

#[tokio::test]
async fn search_serializes_facets_as_anded_groups_of_ored_facets() {
    let mock = body_mock(SEARCH);
    let client = ModrinthClient::with_hook(mock.clone());

    let facets = vec![
        vec![Facet::ProjectType(ProjectType::Modpack)],
        vec![
            Facet::GameVersion("1.21.4".to_string()),
            Facet::Loader("fabric".to_string()),
        ],
    ];
    client
        .search(SearchQuery {
            query: "",
            limit: 5,
            offset: 0,
            facets: Some(facets),
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().query_param("facets"),
        Some(r#"[["project_type:modpack"],["game_versions:1.21.4","loaders:fabric"]]"#)
    );
}

#[tokio::test]
async fn get_versions_builds_url_and_maps_list() {
    let mock = body_mock(VERSIONS);
    let client = ModrinthClient::with_hook(mock.clone());

    let versions = client
        .get_versions(VersionListQuery {
            project_id: "fabric-api",
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.modrinth.com/v2/project/fabric-api/version"
    );
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].id, "version-id");
    assert_eq!(versions[0].files[0].filename, "fabric-api-0.115.0.jar");
}

#[tokio::test]
async fn get_version_builds_url_and_maps_response() {
    let mock = body_mock(VERSION);
    let client = ModrinthClient::with_hook(mock.clone());

    let version = client
        .get_version(VersionQuery {
            version_id: "version-id",
        })
        .await
        .unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.modrinth.com/v2/version/version-id"
    );
    assert_eq!(version.project_id, "P7dR8mSH");
    assert_eq!(version.version_number, "0.115.0+1.21.4");
    assert!(version.featured);
}

#[tokio::test]
async fn get_loaders_builds_url_and_maps_list() {
    let mock = body_mock(LOADERS);
    let client = ModrinthClient::with_hook(mock.clone());

    let loaders = client.get_loaders().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.modrinth.com/v2/tag/loader"
    );
    assert_eq!(loaders.len(), 2);
    assert_eq!(loaders[0].name, "Fabric");
    assert_eq!(loaders[1].supported_project_types, ["mod", "modpack"]);
}

#[tokio::test]
async fn get_game_versions_builds_url_and_maps_list() {
    let mock = body_mock(GAME_VERSIONS);
    let client = ModrinthClient::with_hook(mock.clone());

    let versions = client.get_game_versions().await.unwrap();

    assert_eq!(
        mock.only_request().url,
        "https://api.modrinth.com/v2/tag/game_version"
    );
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0].version, "1.21.4");
    assert!(versions[0].major);
    assert!(!versions[1].major);
}
