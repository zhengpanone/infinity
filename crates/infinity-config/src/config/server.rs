use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, error::Result, traits::Validate};

/// 服务监听配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
#[non_exhaustive]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 8080,
        }
    }
}

impl Validate for ServerConfig {
    fn validate(&self) -> Result<()> {
        if self.host.trim().is_empty() {
            return Err(ConfigError::validation("server.host must not be empty"));
        }
        if self.port == 0 {
            return Err(ConfigError::validation("server.port must not be 0"));
        }
        Ok(())
    }
}
