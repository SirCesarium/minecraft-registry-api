use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FabricGameVersion {
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Deserialize)]
pub struct FabricLoaderVersion {
    pub separator: String,
    pub build: i64,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[derive(Debug, Deserialize)]
pub struct FabricInstallerVersion {
    pub url: String,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_fabric_game_version_deserialize() {
        let json = serde_json::json!({
            "version": "1.21.4",
            "stable": true
        });
        let v: FabricGameVersion = serde_json::from_value(json).unwrap();
        assert_eq!(v.version, "1.21.4");
        assert!(v.stable);
    }

    #[test]
    fn test_fabric_loader_version_deserialize() {
        let json = serde_json::json!({
            "separator": "-",
            "build": 123,
            "maven": "net.fabricmc:fabric-loader:0.16.0",
            "version": "0.16.0",
            "stable": true
        });
        let l: FabricLoaderVersion = serde_json::from_value(json).unwrap();
        assert_eq!(l.version, "0.16.0");
        assert_eq!(l.build, 123);
    }

    #[test]
    fn test_fabric_installer_version_deserialize() {
        let json = serde_json::json!({
            "url": "https://example.com/installer.jar",
            "maven": "net.fabricmc:fabric-installer:1.1.1",
            "version": "1.1.1",
            "stable": true
        });
        let i: FabricInstallerVersion = serde_json::from_value(json).unwrap();
        assert_eq!(i.version, "1.1.1");
        assert_eq!(i.url, "https://example.com/installer.jar");
    }
}
