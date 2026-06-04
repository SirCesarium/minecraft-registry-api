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
