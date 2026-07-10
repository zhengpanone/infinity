//! Admin 领域模型。
//!
//! 目前仅有示例用的 [`Admin`] 类型，展示如何组合共享领域标识
//! [`UserId`](infinity_common::ids::UserId) / [`TenantId`](infinity_common::ids::TenantId)。
//! 后续真实的管理端实体（角色、权限等）也归入本模块。

pub mod command;
pub mod dto;
pub mod types;
pub mod vo;
