use infinity_error::Result;

use crate::domain::dto::config_group::CreateConfigGroupDTO;
use crate::domain::vo::config_group::ConfigGroupVO;

#[async_trait::async_trait]
pub trait ConfigGroupService: Send + Sync {
    /// 创建系统配置-一级分类
    async fn create(&self, request: CreateConfigGroupDTO) -> Result<ConfigGroupVO>;
}
