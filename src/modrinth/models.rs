use serde::Deserialize;
use serde_json::Value;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub hits: Vec<Hit>,
    pub offset: i64,
    pub limit: i64,
    pub total_hits: i64,
}

#[derive(Debug, Deserialize)]
pub struct Hit {
    pub project_id: String,
    pub project_type: String,
    pub slug: String,
    pub author: String,
    pub author_id: String,
    pub organization: Value,
    pub organization_id: Value,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub display_categories: Vec<String>,
    pub versions: Vec<String>,
    pub downloads: i64,
    pub follows: i64,
    pub icon_url: String,
    pub date_created: String,
    pub date_modified: String,
    pub latest_version: String,
    pub license: String,
    pub client_side: String,
    pub server_side: String,
    pub gallery: Vec<Value>,
    pub featured_gallery: Value,
    pub color: Option<i64>,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Deserialize)]
pub struct Project {
    pub client_side: String,
    pub server_side: String,
    pub game_versions: Vec<String>,
    pub id: String,
    pub slug: String,
    pub project_type: String,
    pub team: String,
    pub organization: Value,
    pub title: String,
    pub description: String,
    pub body: String,
    pub body_url: Value,
    pub published: String,
    pub updated: String,
    pub approved: String,
    pub queued: Value,
    pub status: String,
    pub requested_status: Value,
    pub moderator_message: Value,
    pub license: License,
    pub downloads: i64,
    pub followers: i64,
    pub categories: Vec<String>,
    pub additional_categories: Vec<Value>,
    pub loaders: Vec<String>,
    pub versions: Vec<String>,
    pub icon_url: String,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Vec<Value>,
    pub gallery: Vec<Value>,
    pub color: Option<i64>,
    pub thread_id: String,
    pub monetization_status: String,
}

#[derive(Debug, Deserialize)]
pub struct License {
    pub id: String,
    pub name: String,
    pub url: Value,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Deserialize)]
pub struct Version {
    pub game_versions: Vec<String>,
    pub loaders: Vec<String>,
    pub id: String,
    pub project_id: String,
    pub author_id: String,
    pub featured: bool,
    pub name: String,
    pub version_number: String,
    pub changelog: String,
    pub changelog_url: Value,
    pub date_published: String,
    pub downloads: i64,
    pub version_type: String,
    pub status: String,
    pub requested_status: Value,
    pub files: Vec<File>,
    pub dependencies: Vec<Value>,
}

#[allow(clippy::struct_field_names)]
#[derive(Debug, Deserialize)]
pub struct File {
    pub id: String,
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    pub file_type: Value,
}

#[derive(Debug, Deserialize)]
pub struct Hashes {
    pub sha512: String,
    pub sha1: String,
}

#[derive(Debug, Deserialize)]
pub struct Loader {
    pub icon: String,
    pub name: String,
    pub supported_project_types: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GameVersion {
    pub version: String,
    pub version_type: String,
    pub date: String,
    pub major: bool,
}

#[derive(Debug)]
pub struct SearchQuery<'a> {
    pub query: &'a str,
    pub limit: u32,
    pub offset: u32,
    pub facets: Option<Vec<Vec<Facet>>>,
}

#[derive(Debug)]
pub struct ProjectRef<'a> {
    pub slug: &'a str,
}

#[derive(Debug)]
pub struct VersionListQuery<'a> {
    pub project_id: &'a str,
}

#[derive(Debug)]
pub struct VersionQuery<'a> {
    pub version_id: &'a str,
}

#[derive(Debug)]
pub enum Facet {
    ProjectType(ProjectType),
    Category(String),
    Loader(String),
    GameVersion(String),
    ClientSide(Side),
    ServerSide(Side),
    OpenSource(bool),
    License(String),
    Author(String),
    Custom(String, String),
}

#[derive(Debug)]
pub enum ProjectType {
    Mod,
    Modpack,
    Plugin,
    Datapack,
    ResourcePack,
    Shader,
}

#[derive(Debug)]
pub enum Side {
    Required,
    Optional,
    Unsupported,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mod => write!(f, "mod"),
            Self::Modpack => write!(f, "modpack"),
            Self::Plugin => write!(f, "plugin"),
            Self::Datapack => write!(f, "datapack"),
            Self::ResourcePack => write!(f, "resourcepack"),
            Self::Shader => write!(f, "shader"),
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Optional => write!(f, "optional"),
            Self::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl fmt::Display for Facet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectType(t) => write!(f, "project_type:{t}"),
            Self::Category(c) => write!(f, "categories:{c}"),
            Self::Loader(l) => write!(f, "loaders:{l}"),
            Self::GameVersion(v) => write!(f, "game_versions:{v}"),
            Self::ClientSide(s) => write!(f, "client_side:{s}"),
            Self::ServerSide(s) => write!(f, "server_side:{s}"),
            Self::OpenSource(b) => write!(f, "open_source:{b}"),
            Self::License(l) => write!(f, "license:{l}"),
            Self::Author(a) => write!(f, "author:{a}"),
            Self::Custom(k, v) => write!(f, "{k}:{v}"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_deserialize() {
        let json = serde_json::json!({
            "hits": [{
                "project_id": "A1B2C3D4",
                "project_type": "mod",
                "slug": "sodium",
                "author": "CaffeineMC",
                "author_id": "abc123",
                "organization": null,
                "organization_id": null,
                "title": "Sodium",
                "description": "Modern rendering engine",
                "categories": ["performance"],
                "display_categories": ["performance"],
                "versions": ["1.21.4"],
                "downloads": 1_000_000,
                "follows": 50_000,
                "icon_url": "https://example.com/icon.png",
                "date_created": "2020-01-01T00:00:00Z",
                "date_modified": "2024-01-01T00:00:00Z",
                "latest_version": "1.0.0",
                "license": "MIT",
                "client_side": "required",
                "server_side": "required",
                "gallery": [],
                "featured_gallery": null,
                "color": 16_733_525
            }],
            "offset": 0,
            "limit": 10,
            "total_hits": 1
        });
        let result: SearchResult = serde_json::from_value(json).unwrap();
        assert_eq!(result.total_hits, 1);
        assert_eq!(result.hits[0].slug, "sodium");
        assert_eq!(result.hits[0].project_type, "mod");
        assert_eq!(result.hits[0].color, Some(16_733_525));
    }

    #[test]
    fn test_project_deserialize() {
        let json = serde_json::json!({
            "client_side": "required",
            "server_side": "required",
            "game_versions": ["1.21.4"],
            "id": "A1B2C3D4",
            "slug": "sodium",
            "project_type": "mod",
            "team": "team123",
            "organization": null,
            "title": "Sodium",
            "description": "desc",
            "body": "# Sodium\n\nModern",
            "body_url": null,
            "published": "2020-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z",
            "approved": "2020-01-01T00:00:00Z",
            "queued": null,
            "status": "approved",
            "requested_status": null,
            "moderator_message": null,
            "license": { "id": "MIT", "name": "MIT", "url": null },
            "downloads": 1_000_000,
            "followers": 50_000,
            "categories": ["performance"],
            "additional_categories": [],
            "loaders": ["fabric", "quilt"],
            "versions": ["v1.0.0"],
            "icon_url": "https://example.com/icon.png",
            "issues_url": "https://example.com/issues",
            "source_url": "https://example.com/source",
            "wiki_url": null,
            "discord_url": null,
            "donation_urls": [],
            "gallery": [],
            "color": 16_733_525,
            "thread_id": "thread123",
            "monetization_status": "none"
        });
        let project: Project = serde_json::from_value(json).unwrap();
        assert_eq!(project.slug, "sodium");
        assert_eq!(project.project_type, "mod");
        assert_eq!(project.loaders, vec!["fabric", "quilt"]);
        assert_eq!(project.monetization_status, "none");
        assert_eq!(project.color, Some(16_733_525));
        assert_eq!(
            project.issues_url,
            Some("https://example.com/issues".to_string())
        );
        assert_eq!(project.source_url, Some("https://example.com/source".to_string()));
        assert_eq!(project.wiki_url, None);
        assert_eq!(project.discord_url, None);
    }

    #[test]
    fn test_version_deserialize() {
        let json = serde_json::json!({
            "game_versions": ["1.21.4"],
            "loaders": ["fabric"],
            "id": "ver123",
            "project_id": "proj123",
            "author_id": "auth123",
            "featured": false,
            "name": "Sodium 1.0.0",
            "version_number": "1.0.0",
            "changelog": "Initial release",
            "changelog_url": null,
            "date_published": "2024-01-01T00:00:00Z",
            "downloads": 1000,
            "version_type": "release",
            "status": "listed",
            "requested_status": null,
            "files": [{
                "id": "file123",
                "hashes": {
                    "sha512": "aaaa",
                    "sha1": "bbbb"
                },
                "url": "https://example.com/file.jar",
                "filename": "sodium-1.0.0.jar",
                "primary": true,
                "size": 500_000,
                "file_type": null
            }],
            "dependencies": []
        });
        let version: Version = serde_json::from_value(json).unwrap();
        assert_eq!(version.id, "ver123");
        assert_eq!(version.game_versions, vec!["1.21.4"]);
        assert_eq!(version.files.len(), 1);
        assert!(version.files[0].primary);
    }

    #[test]
    fn test_loader_deserialize() {
        let json = serde_json::json!({
            "icon": "fabric",
            "name": "Fabric",
            "supported_project_types": ["mod", "modpack"]
        });
        let loader: Loader = serde_json::from_value(json).unwrap();
        assert_eq!(loader.name, "Fabric");
    }

    #[test]
    fn test_game_version_deserialize() {
        let json = serde_json::json!({
            "version": "1.21.4",
            "version_type": "release",
            "date": "2024-12-03T13:00:00Z",
            "major": true
        });
        let gv: GameVersion = serde_json::from_value(json).unwrap();
        assert_eq!(gv.version, "1.21.4");
        assert!(gv.major);
    }

    #[test]
    fn test_facet_display() {
        assert_eq!(
            Facet::ProjectType(ProjectType::Mod).to_string(),
            "project_type:mod"
        );
        assert_eq!(
            Facet::ProjectType(ProjectType::Plugin).to_string(),
            "project_type:plugin"
        );
        assert_eq!(
            Facet::Category("performance".into()).to_string(),
            "categories:performance"
        );
        assert_eq!(Facet::Loader("fabric".into()).to_string(), "loaders:fabric");
        assert_eq!(
            Facet::ClientSide(Side::Required).to_string(),
            "client_side:required"
        );
        assert_eq!(
            Facet::ServerSide(Side::Unsupported).to_string(),
            "server_side:unsupported"
        );
        assert_eq!(Facet::OpenSource(true).to_string(), "open_source:true");
        assert_eq!(Facet::License("MIT".into()).to_string(), "license:MIT");
        assert_eq!(
            Facet::Author("CaffeineMC".into()).to_string(),
            "author:CaffeineMC"
        );
        assert_eq!(
            Facet::Custom("key".into(), "val".into()).to_string(),
            "key:val"
        );
    }
}
