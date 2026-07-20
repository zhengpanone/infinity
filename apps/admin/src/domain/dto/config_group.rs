use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use infinity_utils::bool_from_int;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateConfigGroupDTO {
    /// 分类编码
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_code: String,

    /// 分组编码
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub group_code: String,

    /// 分组名称
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub group_name: String,

    /// 分组图标
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub icon: Option<String>,

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
    pub group_desc: Option<String>,

    /// 是否系统内置
    #[schema(example = "1")]
    #[serde(deserialize_with = "bool_from_int")]
    pub is_builtin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateConfigGroupDTO {
    pub id: Uuid,

}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct ConfigGroupQueryDTO {
    #[schema(example = "admin")]
    pub username: Option<String>,
    #[schema(example = "admin@qq.com")]
    pub email: Option<String>,
    #[schema(example = "15527300572")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigGroupSortField {
    #[default]
    Username,
    CreateTime,
}

impl ConfigGroupSortField {
    pub const fn as_sql(self) -> &'static str {
        match self {
            Self::Username => "username",
            Self::CreateTime => "created_at",
        }
    }
}


#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CheckConfigGroupExistsDTO {
    /// 分类编码
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_code: String,

    /// 分组编码
    #[schema(example = "admin@qq.com")]
    #[validate(length(max = 100))]
    pub group_code: String,

   
}