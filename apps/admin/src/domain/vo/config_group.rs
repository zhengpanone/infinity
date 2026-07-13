use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct ConfigGroupVO {
    pub id: Uuid,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfigGroupExistsVO {

    pub username_exists: Option<bool>,

    pub email_exists: Option<bool>,
    
    pub phone_exists: Option<bool>,
}
