//! 应用配置模型。

pub mod ai;
pub mod app;
pub mod database;
pub mod grpc;
pub mod logger;
pub mod redis;
pub mod server;

pub use ai::AiConfig;
pub use app::AppInfo;
pub use database::DatabaseConfig;
pub use grpc::GrpcConfig;
pub use logger::LoggerConfig;
pub use redis::RedisConfig;
pub use server::ServerConfig;

use serde::{Deserialize, Serialize};

use crate::{error::Result, traits::Validate};

/// 完整的应用配置。
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AppConfig {
    pub app: AppInfo,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub ai: AiConfig,

    #[serde(default)]
    pub grpc: Option<GrpcConfig>,

    #[serde(default)]
    pub logger: Option<LoggerConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppInfo::default(),
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            ai: AiConfig::default(),
            grpc: None,
            logger: Some(LoggerConfig::default()),
        }
    }
}

impl Validate for AppConfig {
    fn validate(&self) -> Result<()> {
        self.app.validate()?;
        self.server.validate()?;
        self.database.validate()?;
        self.redis.validate()?;
        self.ai.validate()?;

        if let Some(grpc) = &self.grpc {
            grpc.validate()?;
        }
        if let Some(logger) = &self.logger {
            logger.validate()?;
        }

        Ok(())
    }
}

/// 为旧调用方保留的兼容路径，支持 `config::config::AppConfig`。
pub mod config {
    pub use super::AppConfig;
}
