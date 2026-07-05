//! Infinity Admin Server.
//!
//! 演示如何把 `infinity-config`、`infinity-logger`、`infinity-common`
//! 和 `infinity-utils` 组合到同一个启动流程里。

use std::path::PathBuf;

use infinity_common::ids::{TenantId, UserId};
use infinity_config::{Config, config::AppConfig};
use infinity_error::{InfinityError, Result};
use infinity_logger::{Logger, config::LogLevel};

/// 当前 crate 版本。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 管理员账户（示例领域模型）。
#[derive(Debug)]
struct Admin {
    /// 全局唯一的用户 ID。
    id: UserId,
    /// 所属租户 ID。
    tenant: TenantId,
    /// 登录用户名。
    username: String,
}

impl Admin {
    /// 创建一个新的管理员账户，自动生成用户 ID。
    fn new(username: impl Into<String>, tenant: TenantId) -> Self {
        Self {
            id: UserId::generate(),
            tenant,
            username: username.into(),
        }
    }
}

fn main() -> Result<()> {
    // 1. 先加载配置，再根据配置初始化日志。
    let config = load_config()?;
    init_logger(&config)?;

    tracing::info!(
        version = VERSION,
        app = config.app.name.as_str(),
        "admin server starting"
    );

    tracing::info!(
        host = config.server.host.as_str(),
        port = config.server.port,
        "admin server config loaded"
    );

    // 2. 使用共享领域类型创建租户与默认管理员。
    let tenant = TenantId::generate();
    let admin = Admin::new("root", tenant);

    tracing::info!(
        admin_id = admin.id.as_str(),
        tenant_id = admin.tenant.as_str(),
        username = admin.username.as_str(),
        "default admin account created"
    );

    // 3. 使用纯工具函数统计启动耗时。
    let started = infinity_utils::time::now_millis();
    bootstrap(&config, &admin)?;
    let elapsed = infinity_utils::time::now_millis() - started;

    tracing::info!(elapsed_ms = elapsed, "admin server ready");
    Ok(())
}

fn load_config() -> Result<&'static AppConfig> {
    let config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../configs");
    let config = Config::from_dir(config_dir)?;
    Ok(config)
}

fn init_logger(config: &AppConfig) -> Result<()> {
    let logger_config = config.logger.clone().unwrap_or_default();
    let level = parse_log_level(&logger_config.level)?;

    Logger::builder()
        .level(level)
        .console(true)
        .json(logger_config.json)
        .init()?;
    Ok(())
}

fn parse_log_level(level: &str) -> Result<LogLevel> {
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

/// 执行启动流程中的初始化步骤。
///
/// 这里仅作演示；真实实现可根据配置加载数据库、注册路由等。
fn bootstrap(_config: &AppConfig, admin: &Admin) -> Result<()> {
    tracing::debug!(
        username = admin.username.as_str(),
        "running bootstrap tasks"
    );
    Ok(())
}
