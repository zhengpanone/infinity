use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateDictItemDTO {}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateDictItemDTO {}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct DictItemQueryDTO {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DictItemSortField {
    #[default]
    DictItemCode,
    CreateTime,
}

impl DictItemSortField {
    pub const fn as_sql(self) -> &'static str {
        match self {
            Self::DictItemCode => "category_code",
            Self::CreateTime => "created_at",
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CheckDictItemExistsDTO {
    /// 分类编码
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub category_code: Option<String>,
}
