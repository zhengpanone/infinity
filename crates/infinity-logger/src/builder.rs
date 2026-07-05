//! Logger 构建器。
//!
//! 提供流式 API 来配置和初始化日志系统。

use crate::{
    config::{LogLevel, LoggerConfig, Rotation},
    error::Result,
};

/// 日志构建器
///
/// 使用 Builder 模式配置日志系统的各项参数，最终通过 [`init`](LoggerBuilder::init)
/// 完成初始化，或通过 [`build`](LoggerBuilder::build) 获取配置。
///
/// # Examples
///
/// ```
/// use infinity_logger::{Logger, config::{LogLevel, Rotation}};
///
/// # fn main() -> Result<(), infinity_logger::LoggerError> {
/// Logger::builder()
///     .level(LogLevel::Debug)
///     .console(true)
///     .file(true)
///     .directory("./logs")
///     .filename("app")
///     .rotation(Rotation::Daily)
///     .init()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    config: LoggerConfig,
}

impl LoggerBuilder {
    /// 创建一个使用默认配置的 Builder
    pub fn new() -> Self {
        Self {
            config: LoggerConfig::default(),
        }
    }

    /// 设置全局日志级别
    pub fn level(mut self, level: LogLevel) -> Self {
        self.config.level = level;
        self
    }

    /// 是否启用控制台输出
    pub fn console(mut self, enable: bool) -> Self {
        self.config.console.enabled = enable;
        self
    }

    /// 是否启用 JSON 格式输出
    pub fn json(mut self, enable: bool) -> Self {
        self.config.json.enabled = enable;
        self
    }

    /// 是否启用文件输出
    pub fn file(mut self, enable: bool) -> Self {
        self.config.file.enabled = enable;
        self
    }

    /// 设置日志文件目录
    pub fn directory<S: Into<String>>(mut self, dir: S) -> Self {
        self.config.file.directory = dir.into();
        self
    }

    /// 设置日志文件名前缀
    pub fn filename<S: Into<String>>(mut self, file: S) -> Self {
        self.config.file.filename = file.into();
        self
    }

    /// 设置日志轮转策略
    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.config.file.rotation = rotation;
        self
    }

    /// 直接使用一份完整配置替换当前配置
    pub fn config(mut self, cfg: LoggerConfig) -> Self {
        self.config = cfg;
        self
    }

    /// 消费 Builder，返回最终的 [`LoggerConfig`]
    ///
    /// 该方法不会初始化 subscriber，仅返回配置，便于测试或进一步处理。
    pub fn build(self) -> LoggerConfig {
        self.config
    }

    /// 消费 Builder 并初始化全局 subscriber
    ///
    /// # Errors
    ///
    /// 如果 subscriber 已经初始化，返回 [`crate::LoggerError::AlreadyInitialized`]。
    pub fn init(self) -> Result<()> {
        crate::init::init(&self.config)
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_level() {
        let config = LoggerBuilder::new().build();
        assert_eq!(config.level.as_str(), "info");
    }

    #[test]
    fn test_builder_level() {
        let config = LoggerBuilder::new().level(LogLevel::Debug).build();
        assert_eq!(config.level.as_str(), "debug");
    }

    #[test]
    fn test_builder_file() {
        let config = LoggerBuilder::new()
            .file(true)
            .directory("./test_logs")
            .filename("test")
            .rotation(Rotation::Hourly)
            .build();
        assert!(config.file.enabled);
        assert_eq!(config.file.directory, "./test_logs");
        assert_eq!(config.file.filename, "test");
    }

    #[test]
    fn test_builder_console_toggle() {
        let config = LoggerBuilder::new().console(false).build();
        assert!(!config.console.enabled);
    }

    #[test]
    fn test_builder_json_toggle() {
        let config = LoggerBuilder::new().json(true).build();
        assert!(config.json.enabled);
    }

    #[test]
    fn test_builder_config_replace() {
        let custom = LoggerConfig {
            level: LogLevel::Error,
            ..Default::default()
        };
        let config = LoggerBuilder::new().config(custom).build();
        assert_eq!(config.level.as_str(), "error");
    }
}
