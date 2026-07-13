use crate::domain::dto::config::ConfigSortField;

use crate::domain::dto::dict_item::{CheckDictItemExistsDTO, CreateDictItemDTO, DictItemQueryDTO, DictItemSortField, UpdateDictItemDTO};
use crate::domain::types::ids::DictItemId;
use crate::domain::vo::dict_item::{SysDictItemExistsVO, SysDictItemVO};
use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

#[async_trait::async_trait]
pub trait DictItemService: Send + Sync {
    /// 创建系统配置
    async fn create(&self, request: CreateDictItemDTO) -> Result<SysDictItemVO>;

    /// 删除系统配置
    async fn delete(&self, ids: Vec<DictItemId>) -> Result<()>;

    /// 更新系统配置-二级分类
    async fn update(&self, request: UpdateDictItemDTO) -> Result<SysDictItemVO>;

    /// 分页查询
    async fn page_list(
        &self,
        request: PaginationParams<DictItemQueryDTO, DictItemSortField>,
    ) -> Result<PaginatedData<Vec<SysDictItemVO>>>;

    /// 获取系统配置
    async fn get_by_id(&self, id: DictItemId) -> Result<SysDictItemVO>;

    /// 检查用户名、手机号、邮箱是否存在
    async fn check_exists(&self, request: CheckDictItemExistsDTO) -> Result<SysDictItemExistsVO>;
}
