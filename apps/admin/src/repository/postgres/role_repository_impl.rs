use infinity_error::{ErrorKind, Result, ResultExt};
use sqlx::PgPool;
use infinity_web::{PaginatedData, PaginationParams};
use crate::{
    domain::types::RoleId,
    models::role::Role,
    repository::role_repository::{NewRole, RoleRepository},
};
use crate::domain::dto::role::{CheckRoleExistsDTO, RoleQueryDTO, RoleSortField};
use crate::domain::vo::role::RoleExistsVO;
use crate::repository::role_repository::UpdateRole;

/// `sys_role` 全字段列，展开为字符串字面量，供查询/返回复用。
macro_rules! role_columns {
    () => {
        r#"id, role_code, role_name, role_desc, remark,
           role_type, role_status, order_num,
           is_default, is_protected,
           created_id, created_at, created_by,
           updated_id, updated_at, updated_by,
           is_deleted, deleted_at"#
    };
}

pub struct RoleRepositoryImpl {
    pool: PgPool,
}

impl RoleRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl RoleRepository for RoleRepositoryImpl {
    async fn create(&self, role: NewRole) -> Result<Role> {
        let saved = sqlx::query_as::<_, Role>(concat!(
            r#"INSERT INTO sys_role (
                id, role_code, role_name, role_status, role_type
            ) VALUES (
                $1, $2, $3, $4, $5
            ) RETURNING "#,
            role_columns!()
        ))
        .bind(RoleId::generate().as_uuid())
        .bind(role.role_code)
        .bind(role.role_name)
        .bind(role.role_status)
        .bind(role.role_type)
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to create role")?;
        Ok(saved)
    }

    async fn find_by_id(&self, id: &RoleId) -> Result<Option<Role>> {
        todo!()
    }

    async fn update_by_id(&self, user: UpdateRole) -> Result<Option<Role>> {
        todo!()
    }

    async fn soft_delete(&self, ids: &[RoleId]) -> Result<u64> {
        todo!()
    }

    async fn page_list(&self, query: PaginationParams<RoleQueryDTO, RoleSortField>) -> Result<PaginatedData<Vec<Role>>> {
        todo!()
    }

    async fn check_exists(&self, query: &CheckRoleExistsDTO) -> Result<RoleExistsVO> {
        todo!()
    }
}
