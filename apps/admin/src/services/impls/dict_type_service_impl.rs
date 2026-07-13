use crate::domain::dto::dict_type::{CheckDictTypeExistsDTO, CreateDictTypeDTO, DictTypeQueryDTO, DictTypeSortField, UpdateDictTypeDTO};
use crate::domain::vo::dict_type::{DictTypeExistsVO, DictTypeVO};
use crate::repository::dict_type_repository::DictTypeRepository;
use crate::services::dict_type_service::DictTypeService;
use crate::domain::types::ids::{ConfigId, DictTypeId};
use infinity_web::{PaginatedData, PaginationParams};
use std::sync::Arc;

#[derive(Clone)]
pub struct DictTypeServiceImpl {
    dict_type_repository: Arc<dyn DictTypeRepository + Send + Sync>,
}

impl DictTypeServiceImpl {
    pub fn new(dict_type_repository: Arc<dyn DictTypeRepository + Send + Sync>) -> Self {
        Self { dict_type_repository }
    }
}

#[async_trait::async_trait]
impl DictTypeService for DictTypeServiceImpl {
    async fn create(&self, request: CreateDictTypeDTO) -> infinity_error::Result<DictTypeVO> {
        todo!()
    }

    async fn delete(&self, ids: Vec<DictTypeId>) -> infinity_error::Result<()> {
        todo!()
    }

    async fn update(&self, request: UpdateDictTypeDTO) -> infinity_error::Result<DictTypeVO> {
        todo!()
    }

    async fn page_list(
        &self,
        request: PaginationParams<DictTypeQueryDTO, DictTypeSortField>,
    ) -> infinity_error::Result<PaginatedData<Vec<DictTypeVO>>> {
        todo!()
    }

    async fn get_by_id(&self, id: DictTypeId) -> infinity_error::Result<DictTypeVO> {
        todo!()
    }

    async fn check_exists(
        &self,
        request: CheckDictTypeExistsDTO,
    ) -> infinity_error::Result<DictTypeExistsVO> {
        todo!()
    }
}
