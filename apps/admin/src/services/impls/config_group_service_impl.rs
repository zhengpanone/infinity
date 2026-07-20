use crate::domain::dto::config_group::{CheckConfigGroupExistsDTO, ConfigGroupQueryDTO, ConfigGroupSortField, CreateConfigGroupDTO, UpdateConfigGroupDTO};
use crate::domain::types::ids::ConfigGroupId;
use crate::domain::vo::config_group::{ConfigGroupExistsVO, ConfigGroupVO};
use crate::repository::config_group_repository::{ConfigGroupRepository, NewConfigGroup, UpdateConfigGroup};
use crate::services::config_group_service::ConfigGroupService;
use infinity_web::{PaginatedData, PaginationParams};
use std::sync::Arc;
use infinity_error::InfinityError;

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
        let config_group = self
            .config_group_repository
            .create(NewConfigGroup {
                category_code: request.category_code,
                group_code: request.group_code,
                group_name: request.group_name,
                icon: request.icon,
                order_num: request.order_num,
                remark: request.remark,
                group_desc: request.group_desc,
                is_builtin: request.is_builtin,
            }).await?;
        Ok(config_group.into())
    }

    async fn delete(&self, ids: Vec<ConfigGroupId>) -> infinity_error::Result<()> {
        if ids.is_empty() {
            return Err(InfinityError::validation("ids is empty"));
        }
        let affected = self.config_group_repository.soft_delete(&ids).await?;
        if affected == 0 {
            return Err(InfinityError::not_found("config group not found"));
        }
        Ok(())
    }

    async fn update(&self, request: UpdateConfigGroupDTO) -> infinity_error::Result<ConfigGroupVO> {
        let id = ConfigGroupId::from(request.id);
        self.config_group_repository.find_by_id(&id).await?
            .ok_or_else(|| InfinityError::not_found("config group not found"))?;
        let updated = self.config_group_repository
            .update_by_id(UpdateConfigGroup {
                id,
            })
            .await?
            .ok_or_else(|| InfinityError::not_found("config group not found"))?;
        Ok(updated.into())
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
