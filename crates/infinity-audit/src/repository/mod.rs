//! 审计日志持久化。

pub mod audit_repository;

pub use audit_repository::{AuditRepository, InMemoryAuditRepository, PgAuditRepository};
