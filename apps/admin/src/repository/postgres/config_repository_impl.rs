use crate::domain::dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField};
use crate::domain::types::ids::ConfigId;
use crate::models::config::Config;
use crate::repository::config_repository::{ConfigRepository, NewConfig, UpdateConfig};
use infinity_error::{ErrorKind, Result, ResultExt};
use infinity_web::{PaginatedData, PaginationParams};
use sqlx::PgPool;

/// `sys_config` 全字段列，展开为字符串字面量，供查询/返回复用。
macro_rules! role_columns {
    () => {
        r#"id, category_code, group_code, config_key, config_name, config_value, default_value,
           config_type, value_hint, value_unit, validation_rule, is_visible, is_editable,
           is_builtin,is_encrypted,version,order_num,remark,
           created_id, created_at, created_by,
           updated_id, updated_at, updated_by,
           is_deleted, deleted_at"#
    };
}

pub struct ConfigRepositoryImpl {
    pool: PgPool,
}

impl ConfigRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ConfigRepository for ConfigRepositoryImpl {
    async fn create(&self, config: NewConfig) -> Result<Config> {
        let saved = sqlx::query_as::<_, Config>(concat!(
            r#"INSERT INTO sys_config (
                id, category_code, group_code, config_key, config_name, config_value, default_value,
                config_type, value_hint, value_unit, validation_rule, is_visible, is_editable,
                is_builtin,is_encrypted,version,order_num,remark
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
            ) RETURNING "#,
            role_columns!()
        ))
        .bind(ConfigId::generate().as_uuid())
        .bind(config.category_code)
        .bind(config.group_code)
        .bind(config.config_key)
        .bind(config.config_name)
        .bind(config.config_value)
        .bind(config.default_value)
        .bind(config.config_type)
        .bind(config.value_hint)
        .bind(config.value_unit)
        .bind(config.validation_rule)
        .bind(config.is_visible)
        .bind(config.is_editable)
        .bind(config.is_builtin)
        .bind(config.is_encrypted)
        .bind(config.version)
        .bind(config.order_num)
        .bind(config.remark)
        .fetch_one(&self.pool)
        .await
        .context(ErrorKind::Database, "failed to create role")?;
        Ok(saved)
    }

    async fn update_by_id(&self, user: UpdateConfig) -> Result<Option<Config>> {
        todo!()
    }

    async fn soft_delete(&self, ids: &[ConfigId]) -> Result<u64> {
        todo!()
    }

    async fn find_by_id(&self, id: &ConfigId) -> Result<Option<Config>> {
        todo!()
    }

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<Config>>> {
        todo!()
    }
}
