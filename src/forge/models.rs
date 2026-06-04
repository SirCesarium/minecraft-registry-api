use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ForgePromos {
    pub promos: HashMap<String, String>,
}
