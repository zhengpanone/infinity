use std::sync::Arc;

use sqlx::PgPool;

use crate::repository::{
    config_category_repository::ConfigCategoryRepository,
    postgres::{
        role_repository_impl::RoleRepositoryImpl, user_repository_impl::UserRepositoryImpl,
    },
    role_repository::RoleRepository,
    user_repository::UserRepository,
};
use crate::repository::config_group_repository::ConfigGroupRepository;
use crate::repository::config_repository::ConfigRepository;
use crate::repository::postgres::config_category_repository_impl::ConfigCategoryRepositoryImpl;
use crate::repository::postgres::config_group_repository_impl::ConfigGroupRepositoryImpl;
use crate::repository::postgres::config_repository_impl::ConfigRepositoryImpl;

pub mod config_category_repository;
pub mod config_group_repository;
pub mod config_repository;
pub mod postgres;
pub mod role_repository;
pub mod user_repository;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub role_repository: Arc<dyn RoleRepository + Send + Sync>,
    pub config_category_repository: Arc<dyn ConfigCategoryRepository + Send + Sync>,
    pub config_group_repository: Arc<dyn ConfigGroupRepository + Send + Sync>,
    pub config_repository: Arc<dyn ConfigRepository + Send + Sync>,
}

impl Repositories {
    pub fn new(pool: PgPool) -> Self {
        Self {
            // PgPool 是轻量 clone 的连接池句柄
            // 每个 repository 都持有一份 pool 句柄
            // 最后一个可以直接移动 pool，不用 clone
            user_repository: Arc::new(UserRepositoryImpl::new(pool.clone())),
            role_repository: Arc::new(RoleRepositoryImpl::new(pool.clone())),
            config_category_repository: Arc::new(ConfigCategoryRepositoryImpl::new(pool.clone())),
            config_group_repository: Arc::new(ConfigGroupRepositoryImpl::new(pool.clone())),
            config_repository: Arc::new(ConfigRepositoryImpl::new(pool.clone())),
        }
    }
}
