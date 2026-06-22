use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug)]
pub struct ProjectQuery<'a> {
    pub project: &'a str,
}

#[derive(Debug)]
pub struct BuildQuery<'a> {
    pub project: &'a str,
    pub version: &'a str,
    pub build: i64,
}

#[derive(Debug)]
pub struct BuildDownload<'a> {
    pub project: &'a str,
    pub version: &'a str,
    pub build: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaperProject {
    pub project: PaperProjectInfo,
    pub versions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PaperProjectInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PaperVersion {
    pub version: PaperVersionInfo,
    pub builds: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PaperVersionInfo {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct PaperBuild {
    pub id: i64,
    pub time: String,
    pub channel: String,
    pub commits: Vec<PaperBuildCommit>,
    pub downloads: HashMap<String, PaperBuildFile>,
}

#[derive(Debug, Deserialize)]
pub struct PaperBuildCommit {
    pub sha: String,
    pub time: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PaperBuildFile {
    pub name: String,
    pub checksums: PaperBuildChecksums,
    pub size: i64,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PaperBuildChecksums {
    pub sha256: String,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_project_deserialize() {
        let json = serde_json::json!({
            "project": { "id": "paper", "name": "Paper" },
            "versions": {
                "26.1": ["26.1.2", "26.1.1"],
                "1.21": ["1.21.11"]
            }
        });
        let p: PaperProject = serde_json::from_value(json).unwrap();
        assert_eq!(p.project.id, "paper");
        assert_eq!(p.versions.get("26.1").unwrap()[0], "26.1.2");
    }

    #[test]
    fn test_paper_version_deserialize() {
        let json = serde_json::json!({
            "version": { "id": "26.1.2" },
            "builds": [72, 71, 70]
        });
        let v: PaperVersion = serde_json::from_value(json).unwrap();
        assert_eq!(v.version.id, "26.1.2");
        assert_eq!(v.builds, vec![72, 71, 70]);
    }

    #[test]
    fn test_paper_build_deserialize() {
        let json = serde_json::json!({
            "id": 72,
            "time": "2026-06-19T13:08:47Z",
            "channel": "STABLE",
            "commits": [{
                "sha": "abc123",
                "time": "2026-06-19T13:08:32Z",
                "message": "Fix bug"
            }],
            "downloads": {
                "server:default": {
                    "name": "paper-26.1.2-72.jar",
                    "checksums": { "sha256": "deadbeef" },
                    "size": 52_892_581,
                    "url": "https://fill-data.papermc.io/v1/objects/deadbeef/paper-26.1.2-72.jar"
                }
            }
        });
        let b: PaperBuild = serde_json::from_value(json).unwrap();
        assert_eq!(b.id, 72);
        assert_eq!(b.channel, "STABLE");
        assert_eq!(b.commits[0].sha, "abc123");
        assert_eq!(b.downloads["server:default"].name, "paper-26.1.2-72.jar");
    }
}
