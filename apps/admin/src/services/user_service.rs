use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

use crate::domain::{
    dto::user::{CheckUserExistsDTO, CreateUserDTO, UpdateUserDTO, UserQueryDTO, UserSortField},
    types::ids::UserId,
    vo::user::{UserExistsVO, UserVO},
};
use async_trait::async_trait;

/// 查询单个用户的标识方式。
pub enum UserQuery {
    Id(uuid::Uuid),
    Username(String),
    Email(String),
    Phone(String),
}

#[async_trait]
pub trait UserService: Send + Sync {
    /// 创建用户
    async fn create(&self, request: CreateUserDTO) -> Result<UserVO>;

    /// 更新用户
    async fn update(&self, request: UpdateUserDTO) -> Result<UserVO>;

    /// 删除用户
    async fn delete(&self, ids: Vec<UserId>) -> Result<()>;

    /// 按 ID 或用户名获取用户；不存在时返回 `not_found`。
    async fn get_user(&self, query: UserQuery) -> Result<UserVO>;

    /// 分页查询
    async fn page_list(
        &self,
        request: PaginationParams<UserQueryDTO, UserSortField>,
    ) -> Result<PaginatedData<Vec<UserVO>>>;

    async fn get_by_id(&self, id: UserId) -> Result<UserVO>;

    async fn check_exists(&self, request: CheckUserExistsDTO) -> Result<UserExistsVO>;
}
