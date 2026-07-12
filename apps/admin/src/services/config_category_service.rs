use infinity_error::Result;

use crate::domain::{
    dto::config_category::CreateConfigCategoryDTO, vo::config_category::ConfigCategoryVO,
};

#[async_trait::async_trait]
pub trait ConfigCategoryService: Send + Sync {
    /// 创建系统配置-一级分类
    async fn create(&self, request: CreateConfigCategoryDTO) -> Result<ConfigCategoryVO>;
}
