use crate::{
    domain::dto::role::{CheckRoleExistsDTO, RoleQueryDTO, RoleSortField},
    domain::types::RoleId,
    domain::vo::role::RoleExistsVO,
    enums::role::{RoleStatus, RoleType},
    models::role::Role,
};
use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

pub struct NewRole {
    pub role_name: String,
    pub role_code: String,
    pub role_status: RoleStatus,
    pub role_type: RoleType,
}

pub struct UpdateRole {
    pub role_name: String,
    pub role_code: String,
    pub role_status: RoleStatus,
    pub role_type: RoleType,
}

#[async_trait::async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: NewRole) -> Result<Role>;

    async fn find_by_id(&self, id: &RoleId) -> Result<Option<Role>>;

    async fn update_by_id(&self, user: UpdateRole) -> Result<Option<Role>>;

    async fn soft_delete(&self, ids: &[RoleId]) -> Result<u64>;

    async fn page_list(
        &self,
        query: PaginationParams<RoleQueryDTO, RoleSortField>,
    ) -> Result<PaginatedData<Vec<Role>>>;

    async fn check_exists(&self, query: &CheckRoleExistsDTO) -> Result<RoleExistsVO>;
}
