//! 审计结果、落库策略与字段级变更记录。

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::domain::action::AuditAction;

/// 一次被审计操作的结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditResult {
    /// 操作成功完成。
    Succeeded,
    /// 操作因权限不足被拒绝。
    Denied,
    /// 操作执行失败(业务或系统错误)。
    Failed,
}

impl AuditResult {
    /// 所有结果取值,按声明顺序排列。
    pub const ALL: [AuditResult; 3] = [Self::Succeeded, Self::Denied, Self::Failed];

    /// 返回稳定的小写结果码。
    pub const fn code(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Denied => "denied",
            Self::Failed => "failed",
        }
    }

    /// 从稳定结果码解析;未知返回 `None`。
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|result| result.code() == code)
    }

    /// 从 HTTP 状态码推断审计结果。
    ///
    /// `401`/`403` 视为拒绝,其余 `4xx`/`5xx` 视为失败,`2xx`/`3xx` 视为成功。
    pub const fn from_status_code(status: u16) -> Self {
        match status {
            401 | 403 => Self::Denied,
            400..=599 => Self::Failed,
            _ => Self::Succeeded,
        }
    }
}

impl fmt::Display for AuditResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// 高危操作被拒绝时应触发告警。
pub const fn should_alert(result: AuditResult, action: AuditAction) -> bool {
    matches!(result, AuditResult::Denied) && action.is_sensitive()
}

/// 审计日志的落库策略。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditPolicy {
    /// 尽力记录:落库失败仅记 warn 日志,不影响业务操作。
    BestEffort,
    /// 必须记录:落库失败向上抛错,业务操作应随之失败(合规要求)。
    MustRecord,
}

/// 返回某动作的默认落库策略:高危操作必须记录,其余尽力记录。
pub const fn policy_for(action: AuditAction) -> AuditPolicy {
    if action.is_sensitive() {
        AuditPolicy::MustRecord
    } else {
        AuditPolicy::BestEffort
    }
}

/// 审计详情中的字段值:明文或已脱敏。
///
/// 敏感字段(密码、手机号等)在进入审计日志前必须转换为
/// [`Masked`](RedactedValue::Masked) 或经 [`mask`](crate::mask) 模块脱敏后的明文。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "value")]
pub enum RedactedValue {
    /// 可直接记录的明文值。
    Plain(String),
    /// 已脱敏,原值不落库。
    Masked,
}

impl fmt::Display for RedactedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plain(value) => f.write_str(value),
            Self::Masked => f.write_str("******"),
        }
    }
}

/// 一次更新操作中单个字段的前后变化。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldChange {
    /// 字段名(如 `email`、`role`)。
    pub field: String,
    /// 变更前的值。
    pub before: RedactedValue,
    /// 变更后的值。
    pub after: RedactedValue,
}

impl FieldChange {
    /// 创建明文字段变更。
    pub fn plain(
        field: impl Into<String>,
        before: impl Into<String>,
        after: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            before: RedactedValue::Plain(before.into()),
            after: RedactedValue::Plain(after.into()),
        }
    }

    /// 创建敏感字段变更,前后值都不落库。
    pub fn masked(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            before: RedactedValue::Masked,
            after: RedactedValue::Masked,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_code_round_trips() {
        for result in AuditResult::ALL {
            assert_eq!(AuditResult::from_code(result.code()), Some(result));
        }
        assert_eq!(AuditResult::from_code("unknown"), None);
    }

    #[test]
    fn result_from_status_code_maps_http_semantics() {
        assert_eq!(AuditResult::from_status_code(200), AuditResult::Succeeded);
        assert_eq!(AuditResult::from_status_code(302), AuditResult::Succeeded);
        assert_eq!(AuditResult::from_status_code(401), AuditResult::Denied);
        assert_eq!(AuditResult::from_status_code(403), AuditResult::Denied);
        assert_eq!(AuditResult::from_status_code(404), AuditResult::Failed);
        assert_eq!(AuditResult::from_status_code(500), AuditResult::Failed);
    }

    #[test]
    fn denied_sensitive_actions_alert() {
        assert!(should_alert(AuditResult::Denied, AuditAction::ExportData));
        assert!(should_alert(AuditResult::Denied, AuditAction::GrantRole));
        assert!(!should_alert(AuditResult::Denied, AuditAction::Read));
        assert!(!should_alert(
            AuditResult::Succeeded,
            AuditAction::ExportData
        ));
    }

    #[test]
    fn sensitive_actions_must_record() {
        assert_eq!(policy_for(AuditAction::GrantRole), AuditPolicy::MustRecord);
        assert_eq!(
            policy_for(AuditAction::RefundOrder),
            AuditPolicy::MustRecord
        );
        assert_eq!(policy_for(AuditAction::List), AuditPolicy::BestEffort);
    }

    #[test]
    fn masked_value_never_displays_original() {
        let change = FieldChange::masked("password");
        assert_eq!(change.before.to_string(), "******");
        assert_eq!(change.after.to_string(), "******");
    }

    #[test]
    fn field_change_serializes_with_tagged_values() {
        let change = FieldChange::plain("email", "a@x.com", "b@x.com");
        let json = serde_json::to_value(&change).unwrap();
        assert_eq!(json["field"], "email");
        assert_eq!(json["before"]["kind"], "plain");
        assert_eq!(json["before"]["value"], "a@x.com");

        let masked = serde_json::to_value(FieldChange::masked("password")).unwrap();
        assert_eq!(masked["before"]["kind"], "masked");
        assert!(masked["before"].get("value").is_none());
    }
}
