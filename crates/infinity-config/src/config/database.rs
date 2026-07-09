use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, error::Result, traits::Validate};

/// 数据库连接配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[non_exhaustive]
pub struct DatabaseConfig {
    pub url: String,

    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:30432/infinity".into(),
            max_connections: default_max_connections(),
        }
    }
}

impl Validate for DatabaseConfig {
    fn validate(&self) -> Result<()> {
        if self.url.trim().is_empty() {
            return Err(ConfigError::validation("database.url must not be empty"));
        }
        if self.max_connections == 0 {
            return Err(ConfigError::validation(
                "database.max_connections must be greater than 0",
            ));
        }
        Ok(())
    }
}

const fn default_max_connections() -> u32 {
    10
}
