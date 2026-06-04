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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_paper_project_deserialize() {
        let json = serde_json::json!({
            "project_id": "paper",
            "project_name": "Paper",
            "version_groups": ["1.21"],
            "versions": ["1.21.4"]
        });
        let p: PaperProject = serde_json::from_value(json).unwrap();
        assert_eq!(p.project_id, "paper");
    }

    #[test]
    fn test_paper_build_deserialize() {
        let json = serde_json::json!({
            "project_id": "paper",
            "project_name": "Paper",
            "version": "1.21.4",
            "build": 123,
            "time": "2024-12-03T13:00:00Z",
            "channel": "default",
            "promoted": true,
            "changes": [{
                "commit": "abc123",
                "summary": "Fix bug",
                "message": "Fix a critical bug"
            }],
            "downloads": {
                "application": {
                    "name": "paper-1.21.4-123.jar",
                    "sha256": "deadbeef"
                }
            }
        });
        let b: PaperBuild = serde_json::from_value(json).unwrap();
        assert_eq!(b.build, 123);
        assert!(b.promoted);
        assert_eq!(b.downloads.application.name, "paper-1.21.4-123.jar");
        assert_eq!(b.changes[0].commit, "abc123");
    }
}
