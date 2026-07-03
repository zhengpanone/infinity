//! # infinity-utils
//!
//! Infinity 通用**纯工具**库。
//!
//! 只放**无业务含义**的通用工具（时间、ID 生成、字符串处理、编码等）。
//!
//! ## 边界约定
//!
//! - 本 crate **不得**依赖任何 infinity 业务/领域库（包括 `infinity-common`）。
//! - 依赖方向单向：`infinity-common` 可依赖本库，反之**禁止**。
//!
//! 若你要新增的东西带有业务含义（如 `UserId`、`TenantId`），
//! 应放入 `infinity-common` 而非此处。

pub mod id;
pub mod time;
