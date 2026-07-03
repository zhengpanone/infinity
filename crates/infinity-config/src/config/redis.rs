use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, error::Result, traits::Validate};

/// Redis 连接配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct RedisConfig {
    pub host: String,

    #[serde(default = "default_redis_port")]
    pub port: u16,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: default_redis_port(),
        }
    }
}

impl Validate for RedisConfig {
    fn validate(&self) -> Result<()> {
        if self.host.trim().is_empty() {
            return Err(ConfigError::validation("redis.host must not be empty"));
        }
        if self.port == 0 {
            return Err(ConfigError::validation("redis.port must not be 0"));
        }
        Ok(())
    }
}

const fn default_redis_port() -> u16 {
    6379
}
