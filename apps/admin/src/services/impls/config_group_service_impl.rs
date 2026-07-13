use crate::domain::dto::config_group::{CheckConfigGroupExistsDTO, ConfigGroupQueryDTO, ConfigGroupSortField, CreateConfigGroupDTO, UpdateConfigGroupDTO};
use crate::domain::types::ids::ConfigGroupId;
use crate::domain::vo::config_group::{ConfigGroupExistsVO, ConfigGroupVO};
use crate::repository::config_group_repository::ConfigGroupRepository;
use crate::services::config_group_service::ConfigGroupService;
use infinity_web::{PaginatedData, PaginationParams};
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

    async fn delete(&self, ids: Vec<ConfigGroupId>) -> infinity_error::Result<()> {
        todo!()
    }

    async fn update(&self, request: UpdateConfigGroupDTO) -> infinity_error::Result<ConfigGroupVO> {
        todo!()
    }

    async fn page_list(
        &self,
        request: PaginationParams<ConfigGroupQueryDTO, ConfigGroupSortField>,
    ) -> infinity_error::Result<PaginatedData<Vec<ConfigGroupVO>>> {
        todo!()
    }

    async fn get_by_id(&self, id: ConfigGroupId) -> infinity_error::Result<ConfigGroupVO> {
        todo!()
    }

    async fn check_exists(
        &self,
        request: CheckConfigGroupExistsDTO,
    ) -> infinity_error::Result<ConfigGroupExistsVO> {
        todo!()
    }
}
