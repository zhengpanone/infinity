use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuditFields {
    pub created_id: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_id: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
}
