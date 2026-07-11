//! Admin 领域模型。
//!
//! 领域标识（如 [`UserId`](types::ids::UserId)）定义在 [`types`] 子模块中；
//! 后续真实的管理端实体（角色、权限等）也归入本模块。

pub mod command;
pub mod dto;
pub mod types;
pub mod vo;
