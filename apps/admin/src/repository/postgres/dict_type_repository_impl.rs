use crate::domain::dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField};
use crate::domain::types::ids::ConfigId;
use crate::models::dict_type::SysDictType;
use crate::repository::dict_type_repository::{DictTypeRepository, NewDictType, UpdateDictType};
use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};
use sqlx::PgPool;

/// `sys_config` 全字段列，展开为字符串字面量，供查询/返回复用。
macro_rules! dict_type_columns {
    () => {
        r#"id, category_code, group_code, config_key, config_name, config_value, default_value,
           config_type, value_hint, value_unit, validation_rule, is_visible, is_editable,
           is_builtin,is_encrypted,version,order_num,remark,
           created_id, created_at, created_by,
           updated_id, updated_at, updated_by,
           is_deleted, deleted_at"#
    };
}

pub struct DictTypeRepositoryImpl {
    pool: PgPool,
}

impl DictTypeRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl DictTypeRepository for DictTypeRepositoryImpl {
    async fn create(&self, config: NewDictType) -> Result<SysDictType> {
        todo!()
    }

    async fn update_by_id(&self, user: UpdateDictType) -> Result<Option<SysDictType>> {
        todo!()
    }

    async fn soft_delete(&self, ids: &[ConfigId]) -> Result<u64> {
        todo!()
    }

    async fn find_by_id(&self, id: &ConfigId) -> Result<Option<SysDictType>> {
        todo!()
    }

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<SysDictType>>> {
        todo!()
    }
}
