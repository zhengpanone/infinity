use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use validator::Validate;
use crate::enums::config::{ConfigHint, ConfigType};

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateConfigDTO {
    pub category_code: String,
    pub group_code: String,
    pub config_key: String,
    pub config_name: String,
    pub config_value: Option<String>,
    pub default_value: Option<String>,
    pub config_type: ConfigType,
    pub value_hint: ConfigHint,
    pub value_unit: Option<String>,
    pub validation_rule: Option<String>,
    pub is_visible: bool,
    pub is_editable: bool,
    pub is_builtin: bool,
    pub is_encrypted: bool,
    pub version: i64,
    pub order_num: i32,
    pub remark: Option<String>,
    pub options: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateConfigDTO {}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct ConfigQueryDTO {
    #[schema(example = "admin")]
    pub username: Option<String>,
    #[schema(example = "admin@qq.com")]
    pub email: Option<String>,
    #[schema(example = "15527300572")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConfigSortField {
    #[default]
    Username,
    CreateTime,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CheckConfigExistsDTO {

    #[schema(example = "admin")]
    #[validate(length(min = 1, max = 100))]
    pub category_code: String,

    #[schema(example = "admin@qq.com")]
    #[validate(length(min = 1, max = 100))]
    pub group_code: String,

    #[schema(example = "15527300577")]
    #[validate(length(min = 1, max = 100))]
    pub config_key: String,

}
