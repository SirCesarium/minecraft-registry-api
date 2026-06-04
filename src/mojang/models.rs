use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MojangRoot {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Deserialize)]
pub struct Version {
    pub id: String,

    #[serde(rename = "type")]
    pub type_field: String,

    pub url: String,
    pub time: String,

    #[serde(rename = "releaseTime")]
    pub release_time: String,

    pub sha1: String,

    #[serde(rename = "complianceLevel")]
    pub compliance_level: i64,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_manifest() {
        let json = serde_json::json!({
            "latest": { "release": "1.21.4", "snapshot": "1.21.4" },
            "versions": [
                {
                    "id": "1.21.4",
                    "type": "release",
                    "url": "https://example.com/1.21.4.json",
                    "time": "2024-12-03T13:00:00+00:00",
                    "releaseTime": "2024-12-03T13:00:00+00:00",
                    "sha1": "abc123",
                    "complianceLevel": 1
                }
            ]
        });
        let root: MojangRoot = serde_json::from_value(json).unwrap();
        assert_eq!(root.latest.release, "1.21.4");
        assert_eq!(root.versions.len(), 1);
        assert_eq!(root.versions[0].id, "1.21.4");
        assert_eq!(root.versions[0].type_field, "release");
        assert_eq!(root.versions[0].compliance_level, 1);
    }
}
