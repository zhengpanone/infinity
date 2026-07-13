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

pub struct NewConfig {

    /// 一级分类编码
    pub category_code: String,

    /// 二级分组编码
    pub group_code: String,

    /// 配置Key
    pub config_key: String,

    /// 配置名称
    pub config_name: String,

    /// 配置值
    pub config_value: Option<String>,

    /// 默认值
    pub default_value: Option<String>,

    /// 配置类型
    pub config_type: ConfigType,

    /// 前端展示类型
    pub value_hint: ConfigHint,

    /// 单位
    pub value_unit: Option<String>,

    /// 校验规则
    pub validation_rule: Option<String>,

    /// 下拉选项
    pub options: Value,

    /// 是否显示
    pub is_visible: bool,

    /// 是否可编辑
    pub is_editable: bool,

    /// 是否系统内置
    pub is_builtin: bool,

    /// 是否加密
    pub is_encrypted: bool,

    /// 乐观锁版本
    pub version: i64,

    /// 排序
    pub order_num: i32,

    /// 备注
    pub remark: Option<String>,
}

pub struct UpdateConfig {
    pub id: ConfigCategoryId,

    /// 一级分类编码
    pub category_code: String,

    /// 二级分组编码
    pub group_code: String,

    /// 配置Key
    pub config_key: String,

    /// 配置名称
    pub config_name: String,

    /// 配置值
    pub config_value: Option<String>,

    /// 默认值
    pub default_value: Option<String>,

    /// 配置类型
    pub config_type: ConfigType,

    /// 前端展示类型
    pub value_hint: ConfigHint,

    /// 单位
    pub value_unit: Option<String>,

    /// 校验规则
    pub validation_rule: Option<String>,

    /// 下拉选项
    pub options: Value,

    /// 是否显示
    pub is_visible: bool,

    /// 是否可编辑
    pub is_editable: bool,

    /// 是否系统内置
    pub is_builtin: bool,

    /// 是否加密
    pub is_encrypted: bool,

    /// 乐观锁版本
    pub version: i64,

    /// 排序
    pub order_num: i32,

    /// 备注
    pub remark: Option<String>,
}

#[async_trait::async_trait]
pub trait ConfigRepository: Send + Sync {

    async fn create(&self, param: NewConfig) -> Result<Config>;

    async fn update_by_id(&self, user: UpdateConfig) -> Result<Option<Config>>;

    async fn soft_delete(&self, ids: &[ConfigId]) -> Result<u64>;

    async fn find_by_id(&self, id: &ConfigId) -> Result<Option<Config>>;

    async fn page_list(
        &self,
        query: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<Config>>>;
}
