use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use crate::domain::types::RoleId;
use crate::enums::role::{RoleStatus, RoleType};

#[derive(Debug, Clone, FromRow)]
pub struct Role {
    pub id: RoleId,
    pub role_code: String,
    pub role_name: String,
    pub role_status: RoleStatus,
    pub role_type: RoleType,
    pub order_num: i32,
    pub remark: String,
    pub role_desc: String,
    pub is_default: bool,
    pub is_protected: bool,
    pub created_id: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_id: String,
    pub updated_at: DateTime<Utc>,
    pub updated_by: String,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
}
