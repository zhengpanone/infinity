use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use infinity_utils::bool_from_int;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateConfigCategoryDTO {
    /// 分类编码
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_code: String,

    /// 分类名称
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_name: String,

    /// 分类图标
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub icon: Option<String>,

    /// 分类主题色
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub color: Option<String>,

    /// 排序
    #[schema(example = "1")]
    pub order_num: i32,

    /// 备注
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub remark: Option<String>,

    /// 分类描述
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_desc: Option<String>,

    /// 是否系统内置
    #[schema(example = "1")]
    #[serde(deserialize_with = "bool_from_int")]
    pub is_builtin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateConfigCategoryDTO {
    /// 分类ID
    pub id: Uuid,

    /// 分类编码
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_code: Option<String>,

    /// 分类名称
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_name: Option<String>,

    /// 分类图标
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub icon: Option<String>,

    /// 分类主题色
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub color: Option<String>,

    /// 排序
    #[schema(example = "1")]
    pub order_num: Option<i32>,

    /// 备注
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub remark: Option<String>,

    /// 分类描述
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_desc: Option<String>,

    /// 是否系统内置
    #[schema(example = "1")]
    pub is_builtin: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct ConfigCategoryQueryDTO {
    #[schema(example = "admin")]
    pub category_code: Option<String>,

    #[schema(example = "admin@qq.com")]
    pub category_name: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigCategorySortField {
    #[default]
    CategoryCode,
    CreateTime,
}

impl ConfigCategorySortField {
    pub const fn as_sql(self) -> &'static str {
        match self {
            Self::CategoryCode => "category_code",
            Self::CreateTime => "created_at",
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CheckConfigCategoryExistsDTO {
    /// 分类编码
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_code: Option<String>,
}
