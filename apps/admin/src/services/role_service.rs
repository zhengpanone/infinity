use infinity_error::Result;

use crate::domain::{dto::role::CreateRoleDTO, vo::role::RoleVO};

#[async_trait::async_trait]
pub trait RoleService: Send + Sync {
    /// 创建角色
    async fn create(&self, request: CreateRoleDTO) -> Result<RoleVO>;
}
