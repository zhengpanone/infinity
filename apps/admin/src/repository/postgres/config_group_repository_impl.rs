use crate::domain::dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField};
use crate::domain::types::ids::{ConfigCategoryId, ConfigGroupId};
use crate::models::config_group::ConfigGroup;
use crate::repository::config_group_repository::{
    ConfigGroupRepository, NewConfigGroup, UpdateConfigGroup,
};
use infinity_error::{ErrorKind, Result, ResultExt};
use infinity_web::{PaginatedData, PaginationParams};
use sqlx::PgPool;

/// `sys_role` 全字段列，展开为字符串字面量，供查询/返回复用。
macro_rules! role_columns {
    () => {
        r#"id, category_code, group_code, group_name, icon,
           order_num, remark, group_desc, is_builtin,
           created_id, created_at, created_by,
           updated_id, updated_at, updated_by,
           is_deleted, deleted_at"#
    };
}

pub struct ConfigGroupRepositoryImpl {
    pool: PgPool,
}

impl ConfigGroupRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ConfigGroupRepository for ConfigGroupRepositoryImpl {
    async fn create(&self, config_group: NewConfigGroup) -> Result<ConfigGroup> {
        let saved = sqlx::query_as::<_, ConfigGroup>(concat!(
            r#"INSERT INTO sys_config_group (
                id, category_code, group_code, group_name, icon, order_num, remark, group_desc, is_builtin
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            ) RETURNING "#,
            role_columns!()
        ))
        .bind(ConfigCategoryId::generate().as_uuid())
        .bind(config_group.category_code)
        .bind(config_group.group_code)
        .bind(config_group.group_name)
        .bind(config_group.icon)
        .bind(config_group.order_num)
        .bind(config_group.remark)
        .bind(config_group.group_desc)
        .bind(config_group.is_builtin)
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to create role")?;
        Ok(saved)
    }

    async fn update_by_id(&self, config_group: UpdateConfigGroup) -> Result<Option<ConfigGroup>> {
        todo!()
    }

    async fn soft_delete(&self, ids: &[ConfigGroupId]) -> Result<u64> {
        todo!()
    }

    async fn find_by_id(&self, id: &ConfigGroupId) -> Result<Option<ConfigGroup>> {
        todo!()
    }

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<ConfigGroup>>> {
        todo!()
    }
}
