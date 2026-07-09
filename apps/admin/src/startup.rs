//! 启动装配。
//!
//! 负责进程启动早期的一次性准备工作：加载配置、初始化日志、执行启动任务。
//! 这些步骤都以 [`InfinityError`](infinity_error::InfinityError) 传播错误，
//! 由 `main` 统一交给 [`crate::telemetry::report_fatal`] 处理。

use std::path::PathBuf;

use infinity_config::{Config, config::AppConfig};
use infinity_database::Database;
use infinity_error::{InfinityError, Result};
use infinity_logger::{Logger, config::LogLevel};

use crate::domain::Admin;

/// 从工作区 `configs/` 目录加载并校验应用配置，返回进程级 `'static` 引用。
pub fn load_config() -> Result<&'static AppConfig> {
    let config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./configs");
    let config = Config::from_dir(config_dir)?;
    Ok(config)
}

/// 根据配置初始化全局日志订阅器（控制台 + 可选 JSON）。
pub fn init_logger(config: &AppConfig) -> Result<()> {
    let logger_config = config.logger.clone().unwrap_or_default();
    let level = parse_log_level(&logger_config.level)?;

    Logger::builder()
        .level(level)
        .console(true)
        .json(logger_config.json)
        .init()?;
    Ok(())
}

/// 连接数据库并运行迁移。
///
/// 连接或迁移失败都会以 [`InfinityError`] 传播，交由 `main` 统一上报。
pub async fn init_database(config: &AppConfig) -> Result<Database> {
    let db = Database::connect(&config.database).await?;
    let migrate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("./migrations");
    db.migrate(&migrate_dir).await?;
    tracing::info!(
        max_connections = config.database.max_connections,
        "database connected and migrated"
    );
    Ok(db)
}

/// 把配置中的日志级别字符串解析为 [`LogLevel`]；未知级别返回配置错误。
pub(crate) fn parse_log_level(level: &str) -> Result<LogLevel> {
    let parsed = match level.trim().to_ascii_lowercase().as_str() {
        "trace" => LogLevel::Trace,
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        other => {
            return Err(InfinityError::config(format!(
                "unsupported log level: {other}"
            )));
        }
    };

    Ok(parsed)
}

/// 执行启动流程中的初始化任务。
///
/// 这里仅作演示；真实实现可根据配置加载数据库、预热缓存、注册后台任务等。
pub fn bootstrap(_config: &AppConfig, admin: &Admin) -> Result<()> {
    tracing::debug!(username = admin.username(), "running bootstrap tasks");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_log_level_accepts_known_levels_case_insensitively() {
        assert_eq!(parse_log_level("trace").unwrap(), LogLevel::Trace);
        assert_eq!(parse_log_level("DEBUG").unwrap(), LogLevel::Debug);
        assert_eq!(parse_log_level("  Info ").unwrap(), LogLevel::Info);
        assert_eq!(parse_log_level("warn").unwrap(), LogLevel::Warn);
        assert_eq!(parse_log_level("error").unwrap(), LogLevel::Error);
    }

    #[test]
    fn parse_log_level_rejects_unknown_level() {
        let err = parse_log_level("verbose").unwrap_err();
        assert_eq!(err.kind(), infinity_error::ErrorKind::Config);
        assert!(err.to_string().contains("verbose"));
    }
}
