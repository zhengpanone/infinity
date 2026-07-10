use async_trait::async_trait;
use infinity_error::Result;

use crate::{domain::types::username::Username, models::user::User};

pub struct NewUser {
    pub username: Username,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_username(&self, username: &Username) -> Result<Option<User>>;

    async fn create(&self, user: NewUser) -> Result<User>;
}
