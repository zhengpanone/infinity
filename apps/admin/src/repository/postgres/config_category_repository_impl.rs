use crate::domain::dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField};
use crate::domain::types::ids::ConfigCategoryId;
use crate::repository::config_category_repository::UpdateConfigCategory;
use crate::repository::config_category_repository::{ConfigCategoryRepository, NewConfigCategory};
use infinity_error::{ErrorKind, Result, ResultExt};
use infinity_web::{PaginatedData, PaginationParams};
use serde_json::error::Category;
use sqlx::PgPool;

use crate::models::config_category::ConfigCategory;

/// `sys_role` 全字段列，展开为字符串字面量，供查询/返回复用。
macro_rules! role_columns {
    () => {
        r#"id, category_code, category_name, icon, color,
           order_num, remark, category_desc, is_builtin,
           created_id, created_at, created_by,
           updated_id, updated_at, updated_by,
           is_deleted, deleted_at"#
    };
}

pub struct ConfigCategoryRepositoryImpl {
    pool: PgPool,
}

impl ConfigCategoryRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ConfigCategoryRepository for ConfigCategoryRepositoryImpl {
    async fn create(&self, config_category: NewConfigCategory) -> Result<ConfigCategory> {
        let saved = sqlx::query_as::<_, ConfigCategory>(concat!(
            r#"INSERT INTO sys_config_category (
                id, category_code, category_name, icon, color, order_num, remark,
                category_desc, is_builtin,
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            ) RETURNING "#,
            role_columns!()
        ))
        .bind(ConfigCategoryId::generate().as_uuid())
        .bind(config_category.category_code)
        .bind(config_category.category_name)
        .bind(config_category.icon)
        .bind(config_category.color)
        .bind(config_category.order_num)
        .bind(config_category.remark)
        .bind(config_category.category_desc)
        .bind(config_category.is_builtin)
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to create config category")?;
        Ok(saved)
    }

    async fn update_by_id(&self, user: UpdateConfigCategory) -> Result<Option<Category>> {
        todo!()
    }

    async fn soft_delete(&self, ids: &[ConfigCategoryId]) -> Result<u64> {
        todo!()
    }

    async fn find_by_id(&self, id: &ConfigCategoryId) -> Result<Option<ConfigCategory>> {
        todo!()
    }

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<ConfigCategory>>> {
        todo!()
    }
}
