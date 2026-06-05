use serde::Deserialize;

#[derive(Debug)]
pub struct InstallerQuery<'a> {
    pub version: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[serde(rename = "artifactId")]
    pub artifact_id: String,
    pub versioning: Versioning,
}

#[derive(Debug, Deserialize)]
pub struct Versioning {
    pub latest: String,
    pub release: String,
    pub versions: Versions,
}

#[derive(Debug, Deserialize)]
pub struct Versions {
    #[serde(rename = "version")]
    pub list: Vec<String>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_neoforge_metadata_deserialize() {
        let json = serde_json::json!({
            "groupId": "net.neoforged",
            "artifactId": "neoforge",
            "versioning": {
                "latest": "21.4.0-beta",
                "release": "21.4.0-beta",
                "versions": {
                    "version": ["21.4.0-beta", "21.3.0"]
                }
            }
        });
        let m: Metadata = serde_json::from_value(json).unwrap();
        assert_eq!(m.group_id, "net.neoforged");
        assert_eq!(m.versioning.latest, "21.4.0-beta");
        assert_eq!(m.versioning.versions.list.len(), 2);
    }
}
