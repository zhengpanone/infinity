use crate::{
    domain::dto::config::{
        CheckConfigExistsDTO, ConfigQueryDTO, ConfigSortField, CreateConfigDTO, UpdateConfigDTO,
    },
    domain::types::ids::ConfigId,
    domain::vo::config::{ConfigExistsVO, ConfigVO},
    repository::config_repository::ConfigRepository,
    services::config_service::ConfigService,
};
use infinity_web::{PaginatedData, PaginationParams};
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

    async fn delete(&self, ids: Vec<ConfigId>) -> infinity_error::Result<()> {
        todo!()
    }

    async fn update(&self, request: UpdateConfigDTO) -> infinity_error::Result<ConfigVO> {
        todo!()
    }

    async fn page_list(
        &self,
        request: PaginationParams<ConfigQueryDTO, ConfigSortField>,
    ) -> infinity_error::Result<PaginatedData<Vec<ConfigVO>>> {
        todo!()
    }

    async fn get_by_id(&self, id: ConfigId) -> infinity_error::Result<ConfigVO> {
        todo!()
    }

    async fn check_exists(
        &self,
        request: CheckConfigExistsDTO,
    ) -> infinity_error::Result<ConfigExistsVO> {
        todo!()
    }
}
