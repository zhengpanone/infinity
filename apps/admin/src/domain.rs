//! Admin 领域模型。
//!
//! 目前仅有示例用的 [`Admin`] 类型，展示如何组合共享领域标识
//! [`UserId`](infinity_common::ids::UserId) / [`TenantId`](infinity_common::ids::TenantId)。
//! 后续真实的管理端实体（角色、权限等）也归入本模块。

use infinity_common::ids::{TenantId, UserId};

/// 管理员账户（示例领域模型）。
#[derive(Debug)]
pub(crate) struct Admin {
    /// 全局唯一的用户 ID。
    id: UserId,
    /// 所属租户 ID。
    tenant: TenantId,
    /// 登录用户名。
    username: String,
}

impl Admin {
    /// 创建一个新的管理员账户，自动生成用户 ID。
    pub(crate) fn new(username: impl Into<String>, tenant: TenantId) -> Self {
        Self {
            id: UserId::generate(),
            tenant,
            username: username.into(),
        }
    }

    /// 用户 ID 的字符串表示。
    pub(crate) fn id(&self) -> &str {
        self.id.as_str()
    }

    /// 租户 ID 的字符串表示。
    pub(crate) fn tenant(&self) -> &str {
        self.tenant.as_str()
    }

    /// 登录用户名。
    pub(crate) fn username(&self) -> &str {
        &self.username
    }
}
