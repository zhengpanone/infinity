use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CommonIdDTO {
    #[validate(length(min = 1, max = 100))]
    pub ids: Vec<Uuid>,
}
