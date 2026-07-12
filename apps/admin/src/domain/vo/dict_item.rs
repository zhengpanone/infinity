use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DictItemVO {
    pub code: String,

    pub label: String,

    pub value: String,

    pub color: Option<String>,

    pub icon: Option<String>,

    pub is_default: bool,
}
