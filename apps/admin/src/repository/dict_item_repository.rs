use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};
use serde_json::Value;

use crate::enums::config::{ConfigHint, ConfigType};
use crate::{
    domain::{
        dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField},
        types::ids::{ConfigCategoryId, ConfigId},
    },
    models::config::Config,
};

pub struct NewDictItem {}

pub struct UpdateDictItem {
    pub id: ConfigCategoryId,
}

#[async_trait::async_trait]
pub trait DictItemRepository: Send + Sync {
    async fn create(&self, param: NewDictItem) -> Result<Config>;

    async fn update_by_id(&self, user: UpdateDictItem) -> Result<Option<Config>>;

    async fn soft_delete(&self, ids: &[ConfigId]) -> Result<u64>;

    async fn find_by_id(&self, id: &ConfigId) -> Result<Option<Config>>;

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<Config>>>;
}
