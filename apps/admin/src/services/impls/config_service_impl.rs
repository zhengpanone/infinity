use crate::domain::dto::config::CreateConfigDTO;
use crate::domain::vo::config::ConfigVO;
use crate::repository::config_repository::ConfigRepository;
use crate::services::config_service::ConfigService;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConfigServiceImpl {
    config_repository: Arc<dyn ConfigRepository + Send + Sync>,
}

impl ConfigServiceImpl {
    pub fn new(config_repository: Arc<dyn ConfigRepository + Send + Sync>) -> Self {
        Self { config_repository }
    }
}

#[async_trait::async_trait]
impl ConfigService for ConfigServiceImpl {
    async fn create(&self, request: CreateConfigDTO) -> infinity_error::Result<ConfigVO> {
        todo!()
    }
}
