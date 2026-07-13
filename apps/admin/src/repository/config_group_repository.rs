use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

use crate::models::config_group::ConfigGroup;
use crate::domain::{
    dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField},
    types::ids::ConfigCategoryId,
};

pub struct NewConfigGroup {

    /// 一级分类编码
    pub category_code: String,

    /// 分组编码
    pub group_code: String,

    /// 分组名称
    pub group_name: String,

    /// 图标
    pub icon: Option<String>,

    /// 排序
    pub order_num: i32,

    /// 备注
    pub remark: Option<String>,

    /// 描述
    pub group_desc: Option<String>,

    /// 是否内置
    pub is_builtin: bool,
}

pub struct UpdateConfigGroup {
    pub id: ConfigCategoryId,
}

#[async_trait::async_trait]
pub trait ConfigGroupRepository: Send + Sync {

    async fn create(&self, param: NewConfigGroup) -> Result<ConfigGroup>;

    async fn update_by_id(&self, user: UpdateConfigGroup) -> Result<Option<ConfigGroup>>;

    async fn soft_delete(&self, ids: &[ConfigCategoryId]) -> Result<u64>;

    async fn find_by_id(&self, id: &ConfigCategoryId) -> Result<Option<ConfigGroup>>;

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<ConfigGroup>>>;
}
