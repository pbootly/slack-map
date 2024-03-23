use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub requires: Vec<String>,
}

impl Default for PackageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageInfo {
    pub fn new() -> Self {
        PackageInfo {
            name: None,
            version: None,
            requires: Vec::new(),
        }
    }
}

