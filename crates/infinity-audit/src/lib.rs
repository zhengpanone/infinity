//! Infinity 审计日志。
//!
//! 记录"谁在什么时候对什么做了什么、结果如何",覆盖安全合规场景:
//!
//! - **领域模型**([`domain`]):[`AuditAction`](domain::AuditAction) 动作分类、
//!   [`AuditResult`](domain::AuditResult) 结果、[`AuditPolicy`](domain::AuditPolicy)
//!   落库策略、[`AuditLog`](domain::AuditLog) 日志实体与
//!   [`AuditRecord`](domain::AuditRecord) 防篡改哈希链;
//! - **脱敏**([`mask`]):手机号、密码等敏感字段在落库前脱敏;
//! - **持久化**([`repository`]):[`AuditRepository`](repository::AuditRepository)
//!   抽象 + PostgreSQL / 内存实现;
//! - **服务**([`service`]):[`AuditService`](service::AuditService) 按策略落库,
//!   高危操作必须记录成功、普通操作尽力而为;
//! - **中间件**([`middleware`]):axum 中间件自动采集请求来源并落库。
//!
//! # 快速上手
//!
//! ```
//! use std::sync::Arc;
//! use infinity_audit::domain::{AuditAction, AuditLog, AuditResult, FieldChange};
//! use infinity_audit::mask::mask_phone;
//! use infinity_audit::repository::InMemoryAuditRepository;
//! use infinity_audit::service::AuditService;
//!
//! # let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
//! # rt.block_on(async {
//! let service = AuditService::new(Arc::new(InMemoryAuditRepository::new()));
//!
//! let log = AuditLog::builder("u-1", "t-1", AuditAction::Update, AuditResult::Succeeded)
//!     .actor("Alice", "admin")
//!     .target("u-2", "Bob", "user")
//!     .description("更新用户手机号")
//!     .change(FieldChange::plain(
//!         "phone",
//!         mask_phone("13812345678"),
//!         mask_phone("13987654321"),
//!     ))
//!     .build();
//!
//! service.record(log).await.unwrap();
//! # });
//! ```

pub mod domain;
pub mod mask;
pub mod middleware;
pub mod repository;
pub mod service;

pub use domain::{
    AuditAction, AuditLog, AuditLogBuilder, AuditPolicy, AuditQuery, AuditRecord, AuditResult,
    FieldChange, RedactedValue, policy_for, should_alert,
};
pub use middleware::{AuditContext, audit_middleware};
pub use repository::{AuditRepository, InMemoryAuditRepository, PgAuditRepository};
pub use service::AuditService;
