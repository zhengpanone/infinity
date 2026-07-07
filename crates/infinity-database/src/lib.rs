//! Infinity 数据库访问层。
//!
//! 封装 PostgreSQL 连接池（基于 `sqlx`）、迁移与仓储访问。所有 `sqlx::Error`
//! 在本 crate 边界统一转换为 [`InfinityError`](infinity_error::InfinityError)，
//! 使上层只依赖工作区统一错误类型（参见 infinity-error 设计文档 §7）。
//!
//! 本层刻意不依赖 `infinity-common` 等领域 crate，保持基础设施纯净；
//! 数据库记录（如 [`AdminRecord`](repository::AdminRecord)）与领域模型之间的映射由
//! 应用层负责。

pub mod health;
pub mod pool;
pub mod repository;

pub use pool::Database;
