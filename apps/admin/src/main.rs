//! Infinity Admin Server.
//!
//! 组合 infinity-* 基础库启动管理端服务：加载配置、初始化日志、创建领域类型，
//! 并发运行 HTTP 与 gRPC 服务，进程退出时按可观测性规范结构化上报致命错误。
//!
//! 模块划分：
//! - [`startup`]：启动装配（配置 / 日志 / 启动任务）
//! - [`domain`]：领域模型
//! - [`server`]：HTTP 与 gRPC 服务运行时
//! - [`telemetry`]：致命错误上报

use std::process::ExitCode;

use infinity_common::ids::TenantId;
use infinity_error::Result;

mod domain;
mod server;
mod startup;
mod telemetry;

use domain::Admin;

/// 当前 crate 版本。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 进程入口：运行启动流程，失败时结构化上报并返回非零退出码。
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            telemetry::report_fatal(&err);
            ExitCode::FAILURE
        }
    }
}

/// 启动流程主体：装配 → 领域初始化 → 并发运行服务。
///
/// 任何步骤失败都会以 [`InfinityError`](infinity_error::InfinityError) 向上传播，
/// 交由 [`telemetry::report_fatal`] 统一记录。
#[tokio::main]
async fn run() -> Result<()> {
    // 1. 先加载配置，再据此初始化日志。
    let config = startup::load_config()?;
    startup::init_logger(config)?;

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
        admin_id = admin.id(),
        tenant_id = admin.tenant(),
        username = admin.username(),
        "default admin account created"
    );

    // 3. 执行启动任务并统计耗时。
    let started = infinity_utils::time::now_millis();
    startup::bootstrap(config, &admin)?;
    let elapsed = infinity_utils::time::now_millis() - started;
    tracing::info!(elapsed_ms = elapsed, "admin bootstrap complete");

    // 4. 并发启动 HTTP 与 gRPC 服务，收到关闭信号后一起优雅退出。
    server::serve_all(config).await?;
    Ok(())
}
