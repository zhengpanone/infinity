//! Infinity Admin Server.
//!
//! 演示如何组合使用 Infinity 的基础库：
//!
//! - [`infinity_logger`]：统一日志初始化与结构化日志
//! - [`infinity_common`]：跨服务共享的领域类型（[`UserId`] / [`TenantId`]）
//! - [`infinity_utils`]：无业务含义的纯工具（时间戳）
//!
//! [`UserId`]: infinity_common::ids::UserId
//! [`TenantId`]: infinity_common::ids::TenantId

use std::error::Error;

use infinity_common::ids::{TenantId, UserId};
use infinity_logger::{Logger, config::LogLevel};

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

fn main() -> Result<(), Box<dyn Error>> {
    // 1. 初始化日志系统（控制台 + Debug 级别）。
    Logger::builder()
        .level(LogLevel::Debug)
        .console(true)
        .init()?;

    tracing::info!(version = infinity_logger::VERSION, "admin server starting");

    // 2. 使用共享领域类型创建租户与默认管理员。
    let tenant = TenantId::generate();
    let admin = Admin::new("root", tenant.clone());

    tracing::info!(
        admin_id = admin.id.as_str(),
        tenant_id = admin.tenant.as_str(),
        username = admin.username.as_str(),
        "default admin account created"
    );

    // 3. 使用纯工具统计启动耗时。
    let started = infinity_utils::time::now_millis();
    bootstrap(&admin)?;
    let elapsed = infinity_utils::time::now_millis() - started;

    tracing::info!(elapsed_ms = elapsed, "admin server ready");
    Ok(())
}

/// 执行启动流程中的初始化步骤。
///
/// 这里仅作演示；真实实现可加载配置、连接数据库、注册路由等。
fn bootstrap(admin: &Admin) -> Result<(), Box<dyn Error>> {
    tracing::debug!(username = admin.username.as_str(), "running bootstrap tasks");
    Ok(())
}
