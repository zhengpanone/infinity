//! # infinity-common
//!
//! Infinity **共享领域类型**库。
//!
//! 放**跨服务共享的领域类型与 trait**（如 [`UserId`]、[`TenantId`] 等标识类型、
//! 通用上下文、共享枚举）。
//!
//! ## 边界约定
//!
//! - 本 crate 承载**有业务含义**的共享类型；无业务含义的纯工具请放 `infinity-utils`。
//! - 依赖方向单向：本库可依赖 `infinity-utils`，`infinity-utils` **禁止**反向依赖本库。
//!
//! [`UserId`]: ids::UserId
//! [`TenantId`]: ids::TenantId

pub mod ids;
