use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::domain::types::ids::UserId;
use crate::enums::user::UserStatus;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub status: UserStatus,
    pub last_login_at: Option<DateTime<Utc>>,
    pub login_count: i32,
    pub failed_login_count: i32,
    pub last_failed_login_at: Option<DateTime<Utc>>,
    pub is_first_login: bool,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub locked_until: Option<DateTime<Utc>>,
    pub locked_at: Option<DateTime<Utc>>,
    pub lock_reason: Option<String>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub password_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
