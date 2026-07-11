use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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
