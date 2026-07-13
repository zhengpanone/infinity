use crate::domain::{
    dto::config::{
        CheckConfigExistsDTO, ConfigQueryDTO, ConfigSortField, CreateConfigDTO, UpdateConfigDTO,
    },
    types::ids::ConfigId,
    vo::config::{ConfigExistsVO, ConfigVO},
};

use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

#[async_trait::async_trait]
pub trait ConfigService: Send + Sync {
    /// 创建系统配置
    async fn create(&self, request: CreateConfigDTO) -> Result<ConfigVO>;

    /// 删除系统配置
    async fn delete(&self, ids: Vec<ConfigId>) -> Result<()>;

    /// 更新系统配置-二级分类
    async fn update(&self, request: UpdateConfigDTO) -> Result<ConfigVO>;

    /// 分页查询
    async fn page_list(
        &self,
        request: PaginationParams<ConfigQueryDTO, ConfigSortField>,
    ) -> Result<PaginatedData<Vec<ConfigVO>>>;

    /// 获取系统配置
    async fn get_by_id(&self, id: ConfigId) -> Result<ConfigVO>;

    /// 检查用户名、手机号、邮箱是否存在
    async fn check_exists(&self, request: CheckConfigExistsDTO) -> Result<ConfigExistsVO>;
}
