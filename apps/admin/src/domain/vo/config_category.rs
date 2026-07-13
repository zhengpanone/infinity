use crate::models::config_category::ConfigCategory;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct ConfigCategoryVO {
    pub id: Uuid,

    /// 分类编码
    pub category_code: String,

    /// 分类名称
    pub category_name: String,

    /// 分类图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 分类主题色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// 排序
    pub order_num: i32,

    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// 分类描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_desc: Option<String>,

    /// 是否系统内置
    pub is_builtin: bool,

    pub created_id: String,
    pub create_by: String,
    pub created_at: DateTime<Utc>,

    pub updated_id: String,
    pub updated_by: String,
    pub updated_at: DateTime<Utc>,
}

impl From<ConfigCategory> for ConfigCategoryVO {
    fn from(config_category: ConfigCategory) -> Self {
        Self {
            id: config_category.id.as_uuid(),
            category_code: config_category.category_code,
            category_name: config_category.category_name,
            icon: config_category.icon,
            color: config_category.color,
            order_num: config_category.order_num,
            remark: config_category.remark,
            category_desc: config_category.category_desc,
            is_builtin: config_category.is_builtin,
            created_id: config_category.created_id,
            create_by: config_category.created_by,
            created_at: config_category.created_at,
            updated_id: config_category.updated_id,
            updated_by: config_category.updated_by,
            updated_at: config_category.updated_at,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfigCategoryExistsVO {
    pub category_code_exists: bool,
}
