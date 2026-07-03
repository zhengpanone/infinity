use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, error::Result, traits::Validate};

/// 应用基础信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AppInfo {
    pub name: String,

    #[serde(default)]
    pub version: Option<String>,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            name: "Infinity".into(),
            version: None,
        }
    }
}

impl Validate for AppInfo {
    fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            return Err(ConfigError::validation("app.name must not be empty"));
        }
        Ok(())
    }
}
