use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateConfigDTO {}

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
