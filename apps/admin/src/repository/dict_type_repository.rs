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
use crate::models::dict_type::SysDictType;

pub struct NewDictType {



}

pub struct UpdateDictType {

}

#[async_trait::async_trait]
pub trait DictTypeRepository: Send + Sync {

    async fn create(&self, param: NewDictType) -> Result<SysDictType>;

    async fn update_by_id(&self, user: UpdateDictType) -> Result<Option<SysDictType>>;

    async fn soft_delete(&self, ids: &[ConfigId]) -> Result<u64>;

    async fn find_by_id(&self, id: &ConfigId) -> Result<Option<SysDictType>>;

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<SysDictType>>>;
}
