//! Infinity Admin Server.
//!
//! 演示如何把 `infinity-config`、`infinity-logger`、`infinity-common`
//! 和 `infinity-utils` 组合到同一个启动流程里，并按工作区可观测性规范
//! 在进程退出时结构化上报致命错误。

use std::path::PathBuf;
use std::process::ExitCode;

use infinity_common::ids::{TenantId, UserId};
use infinity_config::{Config, config::AppConfig};
use infinity_error::{InfinityError, Result, field};
use infinity_logger::{Logger, config::LogLevel};

mod http;

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

/// 进程入口：运行启动流程，失败时结构化上报并返回非零退出码。
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            report_fatal(&err);
            ExitCode::FAILURE
        }
    }
}

/// 启动流程主体。任何步骤失败都会以 [`InfinityError`] 向上传播，交由
/// [`report_fatal`] 统一记录。
#[tokio::main]
async fn run() -> Result<()> {
    // 1. 先加载配置，再根据配置初始化日志。
    let config = load_config()?;
    init_logger(config)?;

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
    bootstrap(config, &admin)?;
    let elapsed = infinity_utils::time::now_millis() - started;

    tracing::info!(elapsed_ms = elapsed, "admin bootstrap complete");

    // 4. 启动 HTTP 服务，阻塞直到收到关闭信号后优雅退出。
    http::serve(config).await?;
    Ok(())
}

/// 按可观测性规范结构化上报致命错误（参见 infinity-error `docs/OBSERVABILITY.md`）。
///
/// 字段名统一取自 [`infinity_error::field`]，与工作区其余日志保持一致；
/// 同时向 stderr 兜底输出，覆盖「日志尚未初始化」（例如配置加载失败）的窗口。
fn report_fatal(err: &InfinityError) {
    tracing::error!(
        { field::KIND } = err.code(),
        { field::STATUS } = err.status_code(),
        { field::CLASS } = err.class().as_str(),
        { field::ROOT_CAUSE } = %err.root_cause(),
        { field::CHAIN } = err.chain_string(),
        "admin server exited with error"
    );

    // 兜底：配置或日志初始化阶段失败时，tracing 尚无 subscriber，事件会被丢弃。
    eprintln!("fatal [{}]: {}", err.code(), err.chain_string());
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
