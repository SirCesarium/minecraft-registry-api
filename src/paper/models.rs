use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PaperProject {
    pub project_id: String,
    pub project_name: String,
    pub version_groups: Vec<String>,
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PaperBuild {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub build: i64,
    pub time: String,
    pub channel: String,
    pub promoted: bool,
    pub changes: Vec<Change>,
    pub downloads: Downloads,
}

#[derive(Debug, Deserialize)]
pub struct Change {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct Downloads {
    pub application: Application,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub name: String,
    pub sha256: String,
}
