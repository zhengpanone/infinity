use crate::domain::{
    dto::config::{
        CheckConfigExistsDTO, ConfigQueryDTO, ConfigSortField, CreateConfigDTO, UpdateConfigDTO,
    },
    types::ids::ConfigId,
    vo::config::{ConfigExistsVO, ConfigVO},
};

use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};
use crate::domain::dto::dict_type::{CheckDictTypeExistsDTO, CreateDictTypeDTO, DictTypeQueryDTO, DictTypeSortField, UpdateDictTypeDTO};
use crate::domain::types::ids::DictTypeId;
use crate::domain::vo::dict_type::{DictTypeExistsVO, DictTypeVO};

#[async_trait::async_trait]
pub trait DictTypeService: Send + Sync {
    /// 创建系统配置
    async fn create(&self, request: CreateDictTypeDTO) -> Result<DictTypeVO>;

    /// 删除系统配置
    async fn delete(&self, ids: Vec<DictTypeId>) -> Result<()>;

    /// 更新系统配置-二级分类
    async fn update(&self, request: UpdateDictTypeDTO) -> Result<DictTypeVO>;

    /// 分页查询
    async fn page_list(
        &self,
        request: PaginationParams<DictTypeQueryDTO, DictTypeSortField>,
    ) -> Result<PaginatedData<Vec<DictTypeVO>>>;

    /// 获取系统配置
    async fn get_by_id(&self, id: DictTypeId) -> Result<DictTypeVO>;

    /// 检查用户名、手机号、邮箱是否存在
    async fn check_exists(&self, request: CheckDictTypeExistsDTO) -> Result<DictTypeExistsVO>;
}
