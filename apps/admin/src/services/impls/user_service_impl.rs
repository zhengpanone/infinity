use async_trait::async_trait;
use infinity_error::{InfinityError, Result};
use std::sync::Arc;
use tracing::info;

use crate::{
    domain::{command::user::CreateUserCommand, dto::user::CreateUserDTO, vo::user::UserVO},
    repository::user_repository::{NewUser, UserRepository},
    services::user_service::UserService,
};

#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
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

        let password_hash = hash_password(&command.password)?;
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
}

fn hash_password(password: &str) -> Result<String> {
    if password.len() < 8 {
        return Err(InfinityError::validation_field(
            "password",
            "password length must be at least 8",
        ));
    }

    // TODO: replace this with argon2/bcrypt before enabling real authentication.
    Ok(password.to_owned())
}
