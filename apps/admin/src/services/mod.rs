use std::sync::Arc;

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
        Self {
            user_service: Arc::new(UserServiceImpl::new(repositories.user_repository)),
        }
    }
}
