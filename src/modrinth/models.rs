use serde::Deserialize;
use serde_json::Value;

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
    pub color: i64,
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
    pub issues_url: String,
    pub source_url: String,
    pub wiki_url: String,
    pub discord_url: String,
    pub donation_urls: Vec<Value>,
    pub gallery: Vec<Value>,
    pub color: i64,
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
