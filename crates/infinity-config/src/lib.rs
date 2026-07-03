//! Infinity 的分层配置加载库。
//!
//! 支持 TOML / YAML / JSON 单文件加载，支持基础 TOML + 环境覆盖，
//! 提供类型化配置校验，并暴露可选的进程级全局配置。

pub mod builder;
pub mod config;
pub mod configurator;
pub mod environment;
pub mod error;
pub mod loader;
pub mod manager;
pub mod traits;

pub use builder::ConfigBuilder;
pub use config::AppConfig;
pub use configurator::Config;
pub use environment::Environment;
pub use error::{ConfigError, Result};
pub use manager::ConfigManager;
pub use traits::Validate;

/// 当前 crate 版本。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 从默认 `configs` 目录加载并初始化配置。
#[inline]
pub fn init() -> Result<&'static AppConfig> {
    Config::builder().init()
}
