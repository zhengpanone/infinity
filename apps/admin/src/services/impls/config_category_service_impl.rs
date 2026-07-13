use crate::domain::{
    dto::config_category::{
        CheckConfigCategoryExistsDTO, ConfigCategoryQueryDTO, ConfigCategorySortField,
        CreateConfigCategoryDTO, UpdateConfigCategoryDTO,
    },
    types::ids::ConfigCategoryId,
    vo::config_category::{ConfigCategoryExistsVO, ConfigCategoryVO},
};
use crate::repository::config_category_repository::{ConfigCategoryRepository, NewConfigCategory};
use crate::services::config_category_service::ConfigCategoryService;
use async_trait::async_trait;
use infinity_web::{PaginatedData, PaginationParams};
use std::sync::Arc;

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
    async fn create(
        &self,
        request: CreateConfigCategoryDTO,
    ) -> infinity_error::Result<ConfigCategoryVO> {
        let config_category = self
            .config_category_repository
            .create(NewConfigCategory {
                category_code: request.category_code,
                category_name: request.category_name,
                icon: request.icon,
                color: request.color,
                order_num: request.order_num,
                remark: request.remark,
                category_desc: request.category_desc,
                is_builtin: request.is_builtin,
            })
            .await?;
        Ok(config_category.into())
    }

    async fn delete(&self, ids: Vec<ConfigCategoryId>) -> infinity_error::Result<()> {
        todo!()
    }

    async fn update(
        &self,
        request: UpdateConfigCategoryDTO,
    ) -> infinity_error::Result<ConfigCategoryVO> {
        todo!()
    }

    async fn page_list(
        &self,
        request: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> infinity_error::Result<PaginatedData<Vec<ConfigCategoryVO>>> {
        todo!()
    }

    async fn get_by_id(&self, id: ConfigCategoryId) -> infinity_error::Result<ConfigCategoryVO> {
        todo!()
    }

    async fn check_exists(
        &self,
        request: CheckConfigCategoryExistsDTO,
    ) -> infinity_error::Result<ConfigCategoryExistsVO> {
        todo!()
    }
}
