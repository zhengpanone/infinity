use std::sync::Arc;

use infinity_auth::{Argon2PasswordHasher, PasswordHasher};

use crate::{
    repository::Repositories,
    services::{
        impls::{role_service_impl::RoleServiceImpl, user_service_impl::UserServiceImpl},
        role_service::RoleService,
        user_service::UserService,
    },
};

pub mod impls;
pub mod role_service;
pub mod user_service;

// Service Factory
#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<dyn UserService + Send + Sync>,
    pub role_service: Arc<dyn RoleService + Send + Sync>,
}

impl Services {
    pub fn new(repositories: Repositories) -> Self {
        let password_hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::default());
        Self {
            user_service: Arc::new(UserServiceImpl::new(
                repositories.user_repository,
                password_hasher,
            )),
            role_service: Arc::new(RoleServiceImpl::new(repositories.role_repository)),
        }
    }
}
