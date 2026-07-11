use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::role::Role;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct RoleVO {
    pub id: Uuid,
    pub role_name: String,
    pub role_code: String,
    pub role_status: String,
    pub role_type: String,
}

impl From<Role> for RoleVO {
    fn from(role: Role) -> Self {
        Self {
            id: role.id.as_uuid(),
            role_name: role.role_name,
            role_code: role.role_code,
            role_status: role.role_status.to_string(),
            role_type: role.role_type.to_string(),
        }
    }
}
