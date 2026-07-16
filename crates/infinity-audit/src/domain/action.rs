//! 审计动作:标识"谁做了什么"中的"什么"。

use std::fmt;

use serde::{Deserialize, Serialize};

/// 审计动作分类。
///
/// 标记为 `#[non_exhaustive]`:业务演进会持续新增动作,下游 `match`
/// 请始终保留 `_` 分支。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum AuditAction {
    /// 创建资源。
    Create,
    /// 更新资源。
    Update,
    /// 删除资源。
    Delete,
    /// 查看单个资源。
    Read,
    /// 列表查询资源。
    List,
    /// 登录。
    Login,
    /// 登出。
    Logout,
    /// 授予角色(高危)。
    GrantRole,
    /// 回收角色(高危)。
    RevokeRole,
    /// 订单退款(高危)。
    RefundOrder,
    /// 导出数据(高危)。
    ExportData,
}

impl AuditAction {
    /// 所有动作,按声明顺序排列,便于遍历与穷尽测试。
    pub const ALL: [AuditAction; 11] = [
        Self::Create,
        Self::Update,
        Self::Delete,
        Self::Read,
        Self::List,
        Self::Login,
        Self::Logout,
        Self::GrantRole,
        Self::RevokeRole,
        Self::RefundOrder,
        Self::ExportData,
    ];

    /// 返回稳定的小写动作码,适合日志字段与数据库列使用。
    pub const fn code(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Read => "read",
            Self::List => "list",
            Self::Login => "login",
            Self::Logout => "logout",
            Self::GrantRole => "grant_role",
            Self::RevokeRole => "revoke_role",
            Self::RefundOrder => "refund_order",
            Self::ExportData => "export_data",
        }
    }

    /// 从稳定动作码解析回动作;未知动作码返回 `None`。
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|action| action.code() == code)
    }

    /// 该动作是否属于高危操作(授权变更、资金操作、数据导出等)。
    ///
    /// 高危操作默认采用 [`AuditPolicy::MustRecord`](crate::domain::result::AuditPolicy)
    /// 落库策略,且在被拒绝时触发告警。
    pub const fn is_sensitive(self) -> bool {
        matches!(
            self,
            Self::GrantRole | Self::RevokeRole | Self::RefundOrder | Self::ExportData
        )
    }
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_round_trips_for_every_action() {
        for action in AuditAction::ALL {
            assert_eq!(AuditAction::from_code(action.code()), Some(action));
        }
        assert_eq!(AuditAction::from_code("does-not-exist"), None);
    }

    #[test]
    fn sensitive_actions_are_flagged() {
        assert!(AuditAction::GrantRole.is_sensitive());
        assert!(AuditAction::ExportData.is_sensitive());
        assert!(!AuditAction::Read.is_sensitive());
        assert!(!AuditAction::Create.is_sensitive());
    }

    #[test]
    fn serde_uses_snake_case_codes() {
        let json = serde_json::to_string(&AuditAction::GrantRole).unwrap();
        assert_eq!(json, "\"grant_role\"");
        let parsed: AuditAction = serde_json::from_str("\"refund_order\"").unwrap();
        assert_eq!(parsed, AuditAction::RefundOrder);
    }
}
