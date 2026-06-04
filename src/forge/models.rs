use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ForgePromos {
    pub promos: HashMap<String, String>,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn test_forge_promos_deserialize() {
        let json = serde_json::json!({
            "promos": {
                "1.21.4-recommended": "54.1.16",
                "1.21.4-latest": "54.1.19"
            }
        });
        let p: ForgePromos = serde_json::from_value(json).unwrap();
        assert_eq!(p.promos.get("1.21.4-recommended").unwrap(), "54.1.16");
    }
}
