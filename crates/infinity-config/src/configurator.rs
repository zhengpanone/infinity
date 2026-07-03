use std::path::Path;

use crate::{
    builder::ConfigBuilder, config::AppConfig, environment::Environment, error::ConfigError,
    error::Result, manager::ConfigManager,
};

/// 配置入口类型。
pub struct Config;

impl Config {
    /// 创建配置构建器。
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// 从默认的 `configs` 目录加载并初始化配置。
    pub fn init() -> Result<&'static AppConfig> {
        Self::builder().init()
    }

    /// 使用已经构造好的配置值初始化。
    pub fn from_config(config: AppConfig) -> Result<&'static AppConfig> {
        Self::builder().config(config).init()
    }

    /// 从指定目录加载并初始化。
    pub fn from_dir(dir: impl AsRef<Path>) -> Result<&'static AppConfig> {
        Self::builder().dir(dir.as_ref().to_path_buf()).init()
    }

    /// 从指定目录按显式环境加载并初始化。
    pub fn from_dir_with_env(
        dir: impl AsRef<Path>,
        env: Environment,
    ) -> Result<&'static AppConfig> {
        Self::builder()
            .dir(dir.as_ref().to_path_buf())
            .env(env)
            .init()
    }

    /// 返回已初始化的全局配置。
    pub fn current() -> Result<&'static AppConfig> {
        ConfigManager::current()
    }

    /// 返回已初始化的全局配置（若存在）。
    pub fn try_current() -> Option<&'static AppConfig> {
        ConfigManager::try_current()
    }

    /// 动态重载暂未实现。
    pub fn reload() -> Result<()> {
        Err(ConfigError::Unsupported(
            "reload not implemented yet".into(),
        ))
    }

    /// 关闭逻辑暂未实现。
    pub fn shutdown() -> Result<()> {
        Err(ConfigError::Unsupported(
            "shutdown not implemented yet".into(),
        ))
    }
}
