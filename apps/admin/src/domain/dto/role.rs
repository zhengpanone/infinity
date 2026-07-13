use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateRoleDTO {
    #[validate(length(min = 3, max = 50))]
    pub role_name: String,

    #[validate(length(max = 50))]
    pub role_code: String,

    #[validate(length(max = 50))]
    pub role_type: String,

    #[validate(length(max = 50))]
    pub role_status: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateRoleDTO {
    pub id: Uuid,
    #[schema(example = "admin")]
    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,
}


#[skip_serializing_none]
#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct RoleQueryDTO {
    #[schema(example = "admin")]
    pub username: Option<String>,
    #[schema(example = "admin@qq.com")]
    pub email: Option<String>,
    #[schema(example = "15527300572")]
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RoleSortField {
    #[default]
    Username,
    CreateTime,
}

impl RoleSortField {
    pub const fn as_sql(self) -> &'static str {
        match self {
            Self::Username => "username",
            Self::CreateTime => "created_at",
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CheckRoleExistsDTO {
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
