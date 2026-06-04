use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PurpurProject {
    pub project: String,
    pub metadata: Metadata,
    pub versions: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub current: String,
}

#[derive(Debug, Deserialize)]
pub struct PurpurVersion {
    pub project: String,
    pub version: String,
    pub builds: Builds,
}

#[derive(Debug, Deserialize)]
pub struct Builds {
    pub latest: String,
    pub all: Vec<String>,
}
