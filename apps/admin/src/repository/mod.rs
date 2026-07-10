use std::sync::Arc;

use sqlx::PgPool;

use crate::repository::{
    postgres::user_repository_impl::UserRepositoryImpl, user_repository::UserRepository,
};

pub mod postgres;
pub mod user_repository;

#[derive(Clone)]
pub struct Repositories {
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl Repositories {
    pub fn new(pool: PgPool) -> Self {
        Self {
            // PgPool 是轻量 clone 的连接池句柄
            // 每个 repository 都持有一份 pool 句柄
            // 最后一个可以直接移动 pool，不用 clone
            user_repository: Arc::new(UserRepositoryImpl::new(pool.clone())),
        }
    }
}
