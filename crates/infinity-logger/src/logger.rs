use crate::{Result, builder::LoggerBuilder, config::LoggerConfig};

/// Logger 入口
pub struct Logger;

impl Logger {
    /// 创建 Builder
    ///
    /// # Examples
    ///
    /// ```
    /// use infinity_logger::Logger;
    ///
    /// # fn main() -> Result<(), infinity_logger::LoggerError> {
    /// Logger::builder()
    ///     .init()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::default()
    }

    /// 使用默认配置初始化
    ///
    /// 等价于 `Logger::builder().init()`
    ///
    /// # Errors
    ///
    /// 如果 subscriber 已经初始化，返回 [`crate::LoggerError::AlreadyInitialized`]。
    ///
    /// # Examples
    ///
    /// ```
    /// use infinity_logger::Logger;
    ///
    /// # fn main() -> Result<(), infinity_logger::LoggerError> {
    /// Logger::init()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn init() -> Result<()> {
        Self::builder().init()
    }

    /// 从配置初始化
    ///
    /// # Examples
    ///
    /// ```
    /// use infinity_logger::{Logger, config::LoggerConfig};
    ///
    /// # fn main() -> Result<(), infinity_logger::LoggerError> {
    /// let config = LoggerConfig::default();
    /// Logger::from_config(config)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_config(config: LoggerConfig) -> Result<()> {
        Self::builder().config(config).init()
    }

    /// 动态重新加载日志配置
    ///
    /// # Note
    ///
    /// 当前版本尚未实现，将在未来版本提供。
    ///
    /// # Errors
    ///
    /// 返回 [`crate::LoggerError::Unsupported`]。
    pub fn reload() -> Result<()> {
        Err(crate::LoggerError::Unsupported(
            "reload not implemented yet".into(),
        ))
    }

    /// 关闭日志系统
    ///
    /// # Note
    ///
    /// 当前版本尚未实现，将在未来版本提供。
    ///
    /// # Errors
    ///
    /// 返回 [`crate::LoggerError::Unsupported`]。
    pub fn shutdown() -> Result<()> {
        Err(crate::LoggerError::Unsupported(
            "shutdown not implemented yet".into(),
        ))
    }
}
