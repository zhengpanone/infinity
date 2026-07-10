use infinity_error::Result;

use crate::domain::{dto::user::CreateUserDTO, vo::user::UserVO};
use async_trait::async_trait;

#[async_trait]
pub trait UserService: Send + Sync {
    /// 创建用户
    async fn create_user(&self, request: CreateUserDTO) -> Result<UserVO>;
}
