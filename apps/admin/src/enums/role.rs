use serde::{Deserialize, Serialize};
use std::fmt::Display;
use utoipa::ToSchema;

/// 角色状态。对应 Postgres 枚举类型 `role_status_enum`。
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "role_status_enum", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum RoleStatus {
    #[default]
    #[serde(rename = "active")]
    Active,

    #[serde(rename = "inactive")]
    Inactive,

    #[serde(rename = "banned")]
    Banned,
}

impl Display for RoleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RoleStatus::Active => "active",
            RoleStatus::Inactive => "inactive",
            RoleStatus::Banned => "banned",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for RoleStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(RoleStatus::Active),
            "inactive" => Ok(RoleStatus::Inactive),
            "banned" => Ok(RoleStatus::Banned),
            other => Err(format!("invalid role status: {other}")),
        }
    }
}

/// 角色类型。对应 Postgres 枚举类型 `role_type_enum`。
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "role_type_enum", rename_all = "snake_case")]
#[serde(rename_all = "lowercase")]
pub enum RoleType {
    #[default]
    #[serde(rename = "system")]
    System,

    #[serde(rename = "business")]
    Business,

    #[serde(rename = "custom")]
    Custom,
}

impl Display for RoleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RoleType::System => "system",
            RoleType::Business => "business",
            RoleType::Custom => "custom",
        };
        f.write_str(s)
    }
}

impl std::str::FromStr for RoleType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(RoleType::System),
            "business" => Ok(RoleType::Business),
            "custom" => Ok(RoleType::Custom),
            other => Err(format!("invalid role type: {other}")),
        }
    }
}
