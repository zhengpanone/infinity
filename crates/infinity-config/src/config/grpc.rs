use serde::{Deserialize, Serialize};

use crate::{error::ConfigError, error::Result, traits::Validate};

/// gRPC 服务配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct GrpcConfig {
    pub host: String,
    pub port: u16,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".into(),
            port: 50051,
        }
    }
}

impl Validate for GrpcConfig {
    fn validate(&self) -> Result<()> {
        if self.host.trim().is_empty() {
            return Err(ConfigError::validation("grpc.host must not be empty"));
        }
        if self.port == 0 {
            return Err(ConfigError::validation("grpc.port must not be 0"));
        }
        Ok(())
    }
}
