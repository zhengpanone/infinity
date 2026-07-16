use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};
use serde_json::error::Category;

use crate::domain::{
    dto::config_category::{ConfigCategoryQueryDTO, ConfigCategorySortField},
    types::ids::ConfigCategoryId,
};
use crate::models::config_category::ConfigCategory;
use crate::repository::user_repository::{CheckUserExists, UserExistsResult};

pub struct NewConfigCategory {
    /// 分类编码
    pub category_code: String,

    /// 分类名称
    pub category_name: String,

    /// 分类图标
    pub icon: Option<String>,

    /// 分类主题色
    pub color: Option<String>,

    /// 排序
    pub order_num: i32,

    /// 备注
    pub remark: Option<String>,

    /// 分类描述
    pub category_desc: Option<String>,

    /// 是否系统内置
    pub is_builtin: bool,
}

pub struct UpdateConfigCategory {
    pub id: ConfigCategoryId,
    /// 分类编码
    pub category_code: Option<String>,

    /// 分类名称
    pub category_name: Option<String>,

    /// 分类图标
    pub icon: Option<String>,

    /// 分类主题色
    pub color: Option<String>,

    /// 排序
    pub order_num: Option<i32>,

    /// 备注
    pub remark: Option<String>,

    /// 分类描述
    pub category_desc: Option<String>,

    /// 是否系统内置
    pub is_builtin: Option<bool>,
}

pub struct CheckConfigCategoryExists {
    pub category_code: Option<String>,
    pub exclude_category_id: Option<ConfigCategoryId>,
}

pub struct ConfigCategoryExistsResult {
    pub category_code_exists: Option<bool>,
}

#[async_trait::async_trait]
pub trait ConfigCategoryRepository: Send + Sync {
    async fn create(&self, param: NewConfigCategory) -> Result<ConfigCategory>;

    async fn update_by_id(&self, user: UpdateConfigCategory) -> Result<Option<ConfigCategory>>;

    async fn soft_delete(&self, ids: &[ConfigCategoryId]) -> Result<u64>;

    async fn find_by_id(&self, id: &ConfigCategoryId) -> Result<Option<ConfigCategory>>;

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<ConfigCategory>>>;

    async fn check_exists(
        &self,
        query: &CheckConfigCategoryExists,
    ) -> Result<ConfigCategoryExistsResult>;
}
