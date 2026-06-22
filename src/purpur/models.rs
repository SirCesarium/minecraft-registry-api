use serde::Deserialize;

#[derive(Debug)]
pub struct VersionQuery<'a> {
    pub version: &'a str,
}

#[derive(Debug)]
pub struct BuildDownload<'a> {
    pub version: &'a str,
    pub build: &'a str,
}

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

#[derive(Debug, Deserialize)]
pub struct PurpurBuildInfo {
    pub project: String,
    pub version: String,
    pub build: String,
    pub result: String,
    pub metadata: Option<BuildMetadata>,
}

#[derive(Debug, Deserialize)]
pub struct BuildMetadata {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_purpur_project_deserialize() {
        let json = serde_json::json!({
            "project": "purpur",
            "metadata": { "current": "1.21.4" },
            "versions": ["1.21.4", "1.21"]
        });
        let p: PurpurProject = serde_json::from_value(json).unwrap();
        assert_eq!(p.project, "purpur");
        assert_eq!(p.metadata.current, "1.21.4");
        assert_eq!(p.versions.len(), 2);
    }

    #[test]
    fn test_purpur_version_deserialize() {
        let json = serde_json::json!({
            "project": "purpur",
            "version": "1.21.4",
            "builds": {
                "latest": "1234",
                "all": ["1234", "1233", "1232"]
            }
        });
        let v: PurpurVersion = serde_json::from_value(json).unwrap();
        assert_eq!(v.project, "purpur");
        assert_eq!(v.builds.latest, "1234");
        assert_eq!(v.builds.all.len(), 3);
    }
}
