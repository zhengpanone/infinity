use crate::repository::config_category_repository::ConfigCategoryRepository;
use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::dto::config_category::CreateConfigCategoryDTO;
use crate::domain::vo::config_category::ConfigCategoryVO;
use crate::services::config_category_service::ConfigCategoryService;

#[derive(Clone)]
pub struct ConfigCategoryServiceImpl {
    config_category_repository: Arc<dyn ConfigCategoryRepository + Send + Sync>,
}

impl ConfigCategoryServiceImpl {
    pub fn new(
        config_category_repository: Arc<dyn ConfigCategoryRepository + Send + Sync>,
    ) -> Self {
        Self {
            config_category_repository,
        }
    }
}

#[async_trait]
impl ConfigCategoryService for ConfigCategoryServiceImpl {
    async fn create(&self, request: CreateConfigCategoryDTO) -> infinity_error::Result<ConfigCategoryVO> {
        todo!()
    }
}
