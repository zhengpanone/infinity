use std::str::FromStr;
use std::sync::Arc;

use infinity_error::{InfinityError, Result};

use crate::{
    domain::{dto::role::CreateRoleDTO, vo::role::RoleVO},
    enums::role::{RoleStatus, RoleType},
    repository::role_repository::{NewRole, RoleRepository},
    services::role_service::RoleService,
};

#[derive(Clone)]
pub struct RoleServiceImpl {
    role_repository: Arc<dyn RoleRepository + Send + Sync>,
}
impl RoleServiceImpl {
    pub fn new(role_repository: Arc<dyn RoleRepository + Send + Sync>) -> Self {
        Self { role_repository }
    }
}

#[async_trait::async_trait]
impl RoleService for RoleServiceImpl {
    async fn create(&self, request: CreateRoleDTO) -> Result<RoleVO> {
        // 解析枚举，失败时给出明确的业务错误（而不是 500）。
        let role_status = RoleStatus::from_str(&request.role_status)
            .map_err(|e| InfinityError::validation_field("role_status", e))?;
        let role_type = RoleType::from_str(&request.role_type)
            .map_err(|e| InfinityError::validation_field("role_type", e))?;

        let role = self
            .role_repository
            .create(NewRole {
                role_name: request.role_name,
                role_code: request.role_code,
                role_status,
                role_type,
            })
            .await?;
        Ok(role.into())
    }
}
