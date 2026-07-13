use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct ConfigVO {
    pub id: Uuid,

    pub created_at: DateTime<Utc>,
    
    pub updated_at: DateTime<Utc>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfigExistsVO {
    pub username_exists: Option<bool>,
    pub email_exists: Option<bool>,
    pub phone_exists: Option<bool>,
}

