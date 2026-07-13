use crate::domain::{
    dto::config_group::{
        CheckConfigGroupExistsDTO, ConfigGroupQueryDTO, ConfigGroupSortField, CreateConfigGroupDTO,
        UpdateConfigGroupDTO,
    },
    types::ids::ConfigGroupId,
    vo::config_group::{ConfigGroupExistsVO, ConfigGroupVO},
};
use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

#[async_trait::async_trait]
pub trait ConfigGroupService: Send + Sync {
    /// 创建系统配置-二级分类
    async fn create(&self, request: CreateConfigGroupDTO) -> Result<ConfigGroupVO>;

    /// 删除系统配置-二级分类
    async fn delete(&self, ids: Vec<ConfigGroupId>) -> Result<()>;

    /// 更新系统配置-二级分类
    async fn update(&self, request: UpdateConfigGroupDTO) -> Result<ConfigGroupVO>;

    /// 分页查询
    async fn page_list(
        &self,
        request: PaginationParams<ConfigGroupQueryDTO, ConfigGroupSortField>,
    ) -> Result<PaginatedData<Vec<ConfigGroupVO>>>;

    /// 获取系统配置-二级分类
    async fn get_by_id(&self, id: ConfigGroupId) -> Result<ConfigGroupVO>;
    /// 检查用户名、手机号、邮箱是否存在
    async fn check_exists(&self, request: CheckConfigGroupExistsDTO)
    -> Result<ConfigGroupExistsVO>;
}
