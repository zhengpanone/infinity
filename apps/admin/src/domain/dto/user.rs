use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDTO {
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[schema(example = "admin@qq.com")]
    #[validate(length(max = 100), email)]
    pub email: String,

    #[schema(example = "15527300577")]
    #[validate(length(max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[schema(example = "admin123")]
    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[schema(example = "管理员")]
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,

    #[schema(example = "http://123.com")]
    #[validate(length(max = 500), url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[schema(example = "[]")]
    #[serde(default)]
    pub role_ids: Vec<Uuid>,

    #[schema(example = "[]")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permission_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserDTO {
    pub id: Uuid,
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,

    #[schema(example = "admin@qq.com")]
    #[validate(length(max = 100), email)]
    pub email: Option<String>,

    #[schema(example = "15527300577")]
    #[validate(length(max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[schema(example = "admin123")]
    #[validate(length(min = 8, max = 128))]
    pub password: Option<String>,

    #[schema(example = "管理员")]
    #[validate(length(min = 1, max = 100))]
    pub display_name: Option<String>,

    #[schema(example = "http://123.com")]
    #[validate(length(max = 500), url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[schema(example = "[]")]
    #[serde(default)]
    pub role_ids: Option<Vec<Uuid>>,

    #[schema(example = "[]")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permission_ids: Vec<Uuid>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UserQueryDTO {
    #[schema(example = "admin")]
    pub username: Option<String>,
    #[schema(example = "admin@qq.com")]
    pub email: Option<String>,
    #[schema(example = "15527300572")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserSortField {
    #[default]
    Username,
    CreateTime,
}

impl UserSortField {
    pub const fn as_sql(self) -> &'static str {
        match self {
            Self::Username => "username",
            Self::CreateTime => "created_at",
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CheckUserExistsDTO {
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,

    #[schema(example = "admin@qq.com")]
    #[validate(length(max = 100), email)]
    pub email: Option<String>,

    #[schema(example = "15527300577")]
    #[validate(length(max = 20))]
    pub phone: Option<String>,

    /// 更新用户时传入，排除当前用户
    pub exclude_user_id: Option<Uuid>,
}
