use crate::domain::{
    dto::config_category::{
        CheckConfigCategoryExistsDTO, ConfigCategoryQueryDTO, ConfigCategorySortField,
        CreateConfigCategoryDTO, UpdateConfigCategoryDTO,
    },
    types::ids::ConfigCategoryId,
    vo::config_category::{ConfigCategoryExistsVO, ConfigCategoryVO},
};
use crate::repository::config_category_repository::{
    CheckConfigCategoryExists, ConfigCategoryRepository, NewConfigCategory, UpdateConfigCategory,
};
use crate::services::config_category_service::ConfigCategoryService;
use async_trait::async_trait;
use infinity_error::{InfinityError, Result};
use infinity_web::{PaginatedData, PaginationParams};
use sqlx::query;
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
    async fn create(&self, request: CreateConfigCategoryDTO) -> Result<ConfigCategoryVO> {
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

    async fn delete(&self, ids: Vec<ConfigCategoryId>) -> Result<()> {
        if ids.is_empty() {
            return Err(InfinityError::validation("ids is empty"));
        }
        let affected = self.config_category_repository.soft_delete(&ids).await?;
        if affected == 0 {
            return Err(InfinityError::not_found("config category not found"));
        }
        Ok(())
    }

    async fn update(&self, request: UpdateConfigCategoryDTO) -> Result<ConfigCategoryVO> {
        let id = ConfigCategoryId::from(request.id);

        self.config_category_repository
            .find_by_id(&id)
            .await?
            .ok_or_else(|| InfinityError::not_found("config category not found"))?;
        let updated = self
            .config_category_repository
            .update_by_id(UpdateConfigCategory {
                id,
                category_code: request.category_code,
                category_name: request.category_name,
                icon: request.icon,
                color: request.color,
                order_num: request.order_num,
                remark: request.remark,
                category_desc: request.category_desc,
                is_builtin: request.is_builtin,
            })
            .await?
            .ok_or_else(|| InfinityError::not_found("config category"))?;
        Ok(updated.into())
    }

    async fn page_list(
        &self,
        request: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> infinity_error::Result<PaginatedData<Vec<ConfigCategoryVO>>> {
        let page = self.config_category_repository.page_list(request).await?;
        Ok(page.map(|config_category| {
            config_category
                .into_iter()
                .map(ConfigCategoryVO::from)
                .collect::<Vec<_>>()
        }))
    }

    async fn get_by_id(&self, id: ConfigCategoryId) -> Result<ConfigCategoryVO> {
        self.config_category_repository
            .find_by_id(&id)
            .await?
            .map(ConfigCategoryVO::from)
            .ok_or_else(|| InfinityError::not_found("config category not found"))
    }

    async fn check_exists(
        &self,
        request: CheckConfigCategoryExistsDTO,
    ) -> Result<ConfigCategoryExistsVO> {
        let CheckConfigCategoryExistsDTO { category_code } = request;

        let category_code = category_code.as_ref().unwrap();
        let query = CheckConfigCategoryExists {
            category_code: Some(category_code.to_string()),
            exclude_category_id: None,
        };

        let result = self.config_category_repository.check_exists(&query).await?;
        Ok(ConfigCategoryExistsVO {
            category_code_exists: result.category_code_exists,
        })
    }
}
