use serde::Deserialize;

use crate::config::{
    AppInfo, ServerConfig, ai::AiConfig, database::DatabaseConfig, redis::RedisConfig,
};

/// 应用配置
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppInfo,

    pub server: ServerConfig,

    pub database: DatabaseConfig,

    pub redis: RedisConfig,

    // pub logger: LoggerConfig,

    // pub grpc: GrpcConfig,
    pub ai: AiConfig,
}
