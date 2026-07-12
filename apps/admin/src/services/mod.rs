use std::sync::Arc;

use infinity_auth::{Argon2PasswordHasher, PasswordHasher};

use crate::services::config_group_service::ConfigGroupService;
use crate::services::config_service::ConfigService;
use crate::services::impls::config_category_service_impl::ConfigCategoryServiceImpl;
use crate::services::impls::config_group_service_impl::ConfigGroupServiceImpl;
use crate::services::impls::config_service_impl::ConfigServiceImpl;
use crate::{
    repository::Repositories,
    services::{
        config_category_service::ConfigCategoryService,
        impls::{role_service_impl::RoleServiceImpl, user_service_impl::UserServiceImpl},
        role_service::RoleService,
        user_service::UserService,
    },
};

pub mod config_category_service;
pub mod config_group_service;
pub mod config_service;
pub mod impls;
pub mod role_service;
pub mod user_service;

// Service Factory
#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<dyn UserService + Send + Sync>,
    pub role_service: Arc<dyn RoleService + Send + Sync>,
    pub config_category_service: Arc<dyn ConfigCategoryService + Send + Sync>,
    pub config_group_service: Arc<dyn ConfigGroupService + Send + Sync>,
    pub config_service: Arc<dyn ConfigService + Send + Sync>,
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
            config_category_service: Arc::new(ConfigCategoryServiceImpl::new(
                repositories.config_category_repository,
            )),
            config_group_service: Arc::new(ConfigGroupServiceImpl::new(
                repositories.config_group_repository,
            )),
            config_service: Arc::new(ConfigServiceImpl::new(repositories.config_repository)),
        }
    }
}
