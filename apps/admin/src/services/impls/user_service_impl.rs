use async_trait::async_trait;
use infinity_auth::PasswordHasher;
use infinity_error::{InfinityError, Result};
use infinity_web::{PaginatedData, PaginationParams};
use std::sync::Arc;
use tracing::info;

use crate::{
    domain::{
        command::user::CreateUserCommand,
        dto::user::{
            CheckUserExistsDTO, CreateUserDTO, UpdateUserDTO, UserQueryDTO, UserSortField,
        },
        types::{Email, Url, ids::UserId, phone_number::Phone, username::Username},
        vo::user::{UserExistsVO, UserVO},
    },
    repository::user_repository::{CheckUserExists, NewUser, UpdateUser, UserRepository},
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
    async fn create(&self, request: CreateUserDTO) -> Result<UserVO> {
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

    async fn update(&self, request: UpdateUserDTO) -> Result<UserVO> {
        let id = UserId::from(request.id);

        self.user_repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| InfinityError::not_found("user"))?;

        let username = request.username.as_deref().map(Username::new).transpose()?;

        let password_hash = request
            .password
            .as_deref()
            .map(|password| self.password_hasher.hash(password))
            .transpose()?;
        let user = self
            .user_repository
            .update_by_id(UpdateUser {
                id,
                username: username,
                email: request.email.map(Email::try_from).transpose()?,
                phone: request.phone.map(|value| Phone::new(&value)).transpose()?,
                password: password_hash,
                display_name: request.display_name,
                avatar_url: request.avatar_url.as_deref().map(Url::new).transpose()?,
            })
            .await?
            .ok_or_else(|| InfinityError::not_found("user"))?;

        Ok(user.into())
    }

    async fn delete(&self, ids: Vec<UserId>) -> Result<()> {
        if ids.is_empty() {
            return Err(InfinityError::validation("至少需要提供一个用户 ID"));
        }

        let affected = self.user_repository.soft_delete(&ids).await?;
        if affected == 0 {
            return Err(InfinityError::not_found("user"));
        }

        Ok(())
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
            UserQuery::Email(_) => {
                todo!()
            }
            UserQuery::Phone(_) => {
                todo!()
            }
        };

        user.map(UserVO::from)
            .ok_or_else(|| InfinityError::not_found("user"))
    }

    async fn page_list(
        &self,
        query: PaginationParams<UserQueryDTO, UserSortField>,
    ) -> Result<PaginatedData<Vec<UserVO>>> {
        let page = self.user_repository.page_list(query).await?;
        Ok(page.map(|users| users.into_iter().map(UserVO::from).collect::<Vec<_>>()))
    }

    async fn get_by_id(&self, id: UserId) -> Result<UserVO> {
        self.user_repository
            .find_by_id(&id)
            .await?
            .map(UserVO::from)
            .ok_or_else(|| InfinityError::not_found("user"))
    }

    async fn check_exists(&self, request: CheckUserExistsDTO) -> Result<UserExistsVO> {
        let CheckUserExistsDTO {
            username,
            email,
            phone,
            exclude_user_id,
        } = request;

        if username.is_none() && email.is_none() && phone.is_none() {
            return Err(InfinityError::validation(
                "用户名、邮箱、手机号至少提供一个",
            ));
        }

        let query = CheckUserExists {
            username: username.as_deref().map(Username::new).transpose()?,

            email: email.map(Email::try_from).transpose()?,

            phone: phone.map(|value| Phone::new(&value)).transpose()?,

            exclude_user_id: exclude_user_id.map(UserId::from),
        };

        let result = self.user_repository.check_exists(&query).await?;

        Ok(UserExistsVO {
            username_exists: result.username_exists,
            email_exists: result.email_exists,
            phone_exists: result.phone_exists,
        })
    }
}
