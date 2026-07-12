use infinity_error::Result;

use crate::domain::dto::config::CreateConfigDTO;
use crate::domain::vo::config::ConfigVO;

#[async_trait::async_trait]
pub trait ConfigService: Send + Sync {
    /// 创建系统配置-一级分类
    async fn create(&self, request: CreateConfigDTO) -> Result<ConfigVO>;
}
