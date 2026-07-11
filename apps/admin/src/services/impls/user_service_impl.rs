use async_trait::async_trait;
use infinity_auth::PasswordHasher;
use infinity_error::{InfinityError, Result};
use std::sync::Arc;
use tracing::info;

use crate::{
    domain::{
        command::user::CreateUserCommand,
        dto::user::CreateUserDTO,
        types::{ids::UserId, username::Username},
        vo::user::UserVO,
    },
    repository::user_repository::{NewUser, UserRepository},
    services::user_service::{UserQuery, UserService},
};

#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl UserServiceImpl {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repository,
            password_hasher,
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, request: CreateUserDTO) -> Result<UserVO> {
        let command = CreateUserCommand::try_from(request)?;
        info!("Creating new user: username={}", command.username);

        if self
            .user_repository
            .find_by_username(&command.username)
            .await?
            .is_some()
        {
            return Err(InfinityError::conflict(format!(
                "user:{}",
                command.username
            )));
        }

        let password_hash = self.password_hasher.hash(&command.password)?;
        let user = self
            .user_repository
            .create(NewUser {
                username: command.username,
                email: command.email,
                phone: command.phone,
                password_hash,
                display_name: command.display_name,
                avatar_url: command.avatar_url,
            })
            .await?;

        Ok(user.into())
    }

    async fn get_user(&self, query: UserQuery) -> Result<UserVO> {
        let user = match query {
            UserQuery::Id(id) => {
                let id = UserId::new(id);
                self.user_repository.find_by_id(&id).await?
            }
            UserQuery::Username(name) => {
                let username = Username::new(&name)?;
                self.user_repository.find_by_username(&username).await?
            }
        };

        user.map(UserVO::from)
            .ok_or_else(|| InfinityError::not_found("user"))
    }
}
