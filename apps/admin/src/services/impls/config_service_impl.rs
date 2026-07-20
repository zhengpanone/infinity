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
use crate::enums::config::{ConfigHint, ConfigType};
use crate::repository::config_repository::NewConfig;

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
        let config = self.config_repository.create(NewConfig{
            category_code: request.category_code,
            group_code: request.group_code,
            config_key: request.config_key,
            config_name: request.config_name,
            config_value: request.config_value,
            default_value: request.default_value,
            config_type: request.config_type,
            value_hint: request.value_hint,
            value_unit: request.value_unit,
            validation_rule: request.validation_rule,
            options: request.options,
            is_visible: request.is_visible,
            is_editable: request.is_editable,
            is_builtin: request.is_builtin,
            is_encrypted: request.is_encrypted,
            version: request.version,
            order_num: request.order_num,
            remark: request.remark,
        }).await?;
        Ok(config.into())
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
