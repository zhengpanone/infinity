use crate::domain::dto::config_group::CreateConfigGroupDTO;
use crate::domain::vo::config_group::ConfigGroupVO;
use crate::repository::config_group_repository::ConfigGroupRepository;
use crate::services::config_group_service::ConfigGroupService;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConfigGroupServiceImpl {
    config_group_repository: Arc<dyn ConfigGroupRepository + Send + Sync>,
}

impl ConfigGroupServiceImpl {
    pub fn new(config_group_repository: Arc<dyn ConfigGroupRepository + Send + Sync>) -> Self {
        Self {
            config_group_repository,
        }
    }
}
#[async_trait::async_trait]
impl ConfigGroupService for ConfigGroupServiceImpl {
    async fn create(&self, request: CreateConfigGroupDTO) -> infinity_error::Result<ConfigGroupVO> {
        todo!()
    }
}
