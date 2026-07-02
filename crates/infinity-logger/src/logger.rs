use crate::Result;
use crate::builder::LoggerBuilder;

/// Logger 入口
pub struct Logger;

impl Logger {
    /// 创建 Builder
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::default()
    }

    /// 默认初始化
    pub fn init() -> Result<()> {
        Self::builder().init()
    }

    pub fn reload() -> Result<()> {
        todo!()
    }

    pub fn shutdown() -> Result<()> {
        todo!()
    }
    pub fn from_config() -> Result<()> {
        todo!()
    }
}
