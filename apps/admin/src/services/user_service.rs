use infinity_error::Result;

use crate::domain::{dto::user::CreateUserDTO, vo::user::UserVO};
use async_trait::async_trait;

/// 查询单个用户的标识方式。
pub enum UserQuery {
    Id(uuid::Uuid),
    Username(String),
}

#[async_trait]
pub trait UserService: Send + Sync {
    /// 创建用户
    async fn create_user(&self, request: CreateUserDTO) -> Result<UserVO>;

    /// 按 ID 或用户名获取用户；不存在时返回 `not_found`。
    async fn get_user(&self, query: UserQuery) -> Result<UserVO>;
}
