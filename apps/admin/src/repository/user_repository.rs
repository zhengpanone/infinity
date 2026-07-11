use async_trait::async_trait;
use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

use crate::{
    domain::{
        dto::user::{UserQueryDTO, UserSortField},
        types::{Email, Url, ids::UserId, phone_number::Phone, username::Username},
    },
    models::user::User,
};

pub struct NewUser {
    pub username: Username,
    pub email: Email,
    pub phone: Option<Phone>,
    pub password_hash: String,
    pub display_name: String,
    pub avatar_url: Option<Url>,
}

pub struct UpdateUser {
    pub id: UserId,
    pub username: Option<Username>,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub password: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<Url>,
}

pub struct CheckUserExists {
    pub username: Option<Username>,
    pub email: Option<Email>,
    pub phone: Option<Phone>,
    pub exclude_user_id: Option<UserId>,
}

pub struct UserExistsResult {
    pub username_exists: Option<bool>,
    pub email_exists: Option<bool>,
    pub phone_exists: Option<bool>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>>;

    async fn find_by_username(&self, username: &Username) -> Result<Option<User>>;

    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;

    async fn create(&self, user: NewUser) -> Result<User>;

    async fn update_by_id(&self, user: UpdateUser) -> Result<Option<User>>;

    async fn soft_delete(&self, ids: &[UserId]) -> Result<u64>;

    async fn page_list(
        &self,
        query: PaginationParams<UserQueryDTO, UserSortField>,
    ) -> Result<PaginatedData<Vec<User>>>;

    async fn check_exists(&self, query: &CheckUserExists) -> Result<UserExistsResult>;
}
