use crate::domain::{
    dto::config_category::{
        CheckConfigCategoryExistsDTO, ConfigCategoryQueryDTO, ConfigCategorySortField,
        CreateConfigCategoryDTO, UpdateConfigCategoryDTO,
    },
    types::ids::ConfigCategoryId,
    vo::config_category::{ConfigCategoryExistsVO, ConfigCategoryVO},
};
use infinity_error::Result;
use infinity_web::{PaginatedData, PaginationParams};

#[async_trait::async_trait]
pub trait ConfigCategoryService: Send + Sync {
    /// 创建系统配置分类
    async fn create(&self, request: CreateConfigCategoryDTO) -> Result<ConfigCategoryVO>;

    /// 删除系统配置分类
    async fn delete(&self, ids: Vec<ConfigCategoryId>) -> Result<()>;

    /// 更新系统配置分类
    async fn update(&self, request: UpdateConfigCategoryDTO) -> Result<ConfigCategoryVO>;

    /// 分页查询
    async fn page_list(
        &self,
        request: PaginationParams<ConfigCategoryQueryDTO, ConfigCategorySortField>,
    ) -> Result<PaginatedData<Vec<ConfigCategoryVO>>>;
    /// 获取系统配置分类
    async fn get_by_id(&self, id: ConfigCategoryId) -> Result<ConfigCategoryVO>;

    /// 检查用户名、手机号、邮箱是否存在
    async fn check_exists(
        &self,
        request: CheckConfigCategoryExistsDTO,
    ) -> Result<ConfigCategoryExistsVO>;
}
