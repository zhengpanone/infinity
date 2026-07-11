use infinity_error::Result;

use crate::enums::role::{RoleStatus, RoleType};
use crate::models::role::Role;

pub struct NewRole {
    pub role_name: String,
    pub role_code: String,
    pub role_status: RoleStatus,
    pub role_type: RoleType,
}

#[async_trait::async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: NewRole) -> Result<Role>;
}
