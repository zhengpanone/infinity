use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, error::Result, traits::Validate};

/// 日志配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct LoggerConfig {
    pub level: String,

    #[serde(default)]
    pub json: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: "info".into(),
            json: false,
        }
    }
}

impl Validate for LoggerConfig {
    fn validate(&self) -> Result<()> {
        let level = self.level.to_ascii_lowercase();
        match level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => Ok(()),
            _ => Err(ConfigError::validation(
                "logger.level must be trace, debug, info, warn, or error",
            )),
        }
    }
}
