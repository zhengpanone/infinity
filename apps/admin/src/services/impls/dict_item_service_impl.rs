use crate::domain::dto::config::ConfigSortField;
use crate::domain::dto::dict_item::{
    CheckDictItemExistsDTO, CreateDictItemDTO, DictItemQueryDTO, DictItemSortField,
    UpdateDictItemDTO,
};
use crate::domain::types::ids::DictItemId;
use crate::domain::vo::dict_item::{SysDictItemExistsVO, SysDictItemVO};
use crate::repository::dict_item_repository::DictItemRepository;
use crate::services::dict_item_service::DictItemService;
use infinity_web::{PaginatedData, PaginationParams};
use std::sync::Arc;

#[derive(Clone)]
pub struct DictItemServiceImpl {
    config_repository: Arc<dyn DictItemRepository + Send + Sync>,
}

impl DictItemServiceImpl {
    pub fn new(config_repository: Arc<dyn DictItemRepository + Send + Sync>) -> Self {
        Self { config_repository }
    }
}

#[async_trait::async_trait]
impl DictItemService for DictItemServiceImpl {
    async fn create(&self, request: CreateDictItemDTO) -> infinity_error::Result<SysDictItemVO> {
        todo!()
    }

    async fn delete(&self, ids: Vec<DictItemId>) -> infinity_error::Result<()> {
        todo!()
    }

    async fn update(&self, request: UpdateDictItemDTO) -> infinity_error::Result<SysDictItemVO> {
        todo!()
    }

    async fn page_list(
        &self,
        request: PaginationParams<DictItemQueryDTO, DictItemSortField>,
    ) -> infinity_error::Result<PaginatedData<Vec<SysDictItemVO>>> {
        todo!()
    }

    async fn get_by_id(&self, id: DictItemId) -> infinity_error::Result<SysDictItemVO> {
        todo!()
    }

    async fn check_exists(
        &self,
        request: CheckDictItemExistsDTO,
    ) -> infinity_error::Result<SysDictItemExistsVO> {
        todo!()
    }
}
