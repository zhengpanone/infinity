use std::sync::Arc;

use infinity_auth::{Argon2PasswordHasher, PasswordHasher};

use crate::{
    repository::Repositories,
    services::{impls::user_service_impl::UserServiceImpl, user_service::UserService},
};

pub mod impls;
pub mod user_service;

// Service Factory
#[derive(Clone)]
pub struct Services {
    pub user_service: Arc<dyn UserService + Send + Sync>,
}

impl Services {
    pub fn new(repositories: Repositories) -> Self {
        let password_hasher: Arc<dyn PasswordHasher> = Arc::new(Argon2PasswordHasher::default());
        Self {
            user_service: Arc::new(UserServiceImpl::new(
                repositories.user_repository,
                password_hasher,
            )),
        }
    }
}
