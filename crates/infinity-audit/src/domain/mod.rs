//! 审计领域模型:动作、结果、策略与日志实体。

pub mod action;
pub mod audit;
pub mod result;

pub use action::AuditAction;
pub use audit::{AuditLog, AuditLogBuilder, AuditQuery, AuditRecord};
pub use result::{
    AuditPolicy, AuditResult, FieldChange, RedactedValue, policy_for, should_alert,
};
