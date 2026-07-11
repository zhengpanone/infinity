use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct UserVO {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    #[serde(default)]
    pub email_verified: bool,
    #[serde(default)]
    pub phone_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<DateTime<Utc>>,
    pub login_count: i64,
    pub failed_login_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl From<User> for UserVO {
    fn from(user: User) -> Self {
        Self {
            id: user.id.as_uuid(),
            username: user.username,
            email: user.email,
            phone: user.phone,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            roles: Vec::new(),
            permissions: Vec::new(),
            email_verified: user.email_verified,
            phone_verified: user.phone_verified,
            last_login_at: user.last_login_at,
            login_count: i64::from(user.login_count),
            failed_login_count: i64::from(user.failed_login_count),
            last_failed_login_at: user.last_failed_login_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
            metadata: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserExistsVO {
    pub username_exists: Option<bool>,
    pub email_exists: Option<bool>,
    pub phone_exists: Option<bool>,
}
