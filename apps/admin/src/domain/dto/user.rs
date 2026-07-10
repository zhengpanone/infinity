use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateUserDTO {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(length(max = 100), email)]
    pub email: String,

    #[validate(length(max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[validate(length(min = 1, max = 100))]
    pub display_name: String,

    #[validate(length(max = 500), url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[serde(default)]
    pub role_ids: Vec<Uuid>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permission_ids: Vec<Uuid>,
}
