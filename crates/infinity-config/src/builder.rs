use std::path::{Path, PathBuf};

use crate::{
    config::AppConfig, environment::Environment, error::Result, loader, manager::ConfigManager,
    traits::Validate,
};

/// 用于加载和初始化应用配置的构建器。
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    config: Option<AppConfig>,
    dir: PathBuf,
    env: Environment,
}

impl ConfigBuilder {
    /// 创建一个构建器，默认从 `configs/application.toml` 按当前环境加载。
    pub fn new() -> Self {
        Self {
            config: None,
            dir: PathBuf::from("configs"),
            env: Environment::detect(),
        }
    }

    /// 使用显式配置值替换当前构建器状态。
    pub fn config(mut self, config: AppConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// 设置包含 `application.toml` 的目录。
    pub fn dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.dir = dir.into();
        self
    }

    /// 设置用于覆盖文件的运行环境。
    pub fn env(mut self, env: Environment) -> Self {
        self.env = env;
        self
    }

    /// 直接加载单个配置文件，不使用基础文件 + 覆盖文件模式。
    pub fn file<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let config: AppConfig = loader::load_file(path)?;
        config.validate()?;
        self.config = Some(config);
        Ok(self)
    }

    /// 消费构建器并返回最终配置。
    pub fn build(self) -> Result<AppConfig> {
        let config = match self.config {
            Some(config) => config,
            None => ConfigManager::load_from_dir_with_env(self.dir, self.env)?,
        };
        config.validate()?;
        Ok(config)
    }

    /// 消费构建器并初始化进程级全局配置。
    pub fn init(self) -> Result<&'static AppConfig> {
        let config = self.build()?;
        ConfigManager::init(config)?;
        ConfigManager::current()
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default_env() {
        let builder = ConfigBuilder::new();
        assert_eq!(builder.dir, PathBuf::from("configs"));
    }

    #[test]
    fn test_builder_config_replace() {
        let config = AppConfig::default();
        let built = ConfigBuilder::new().config(config.clone()).build().unwrap();
        assert_eq!(built.app.name, config.app.name);
    }

    #[test]
    fn test_builder_env() {
        let builder = ConfigBuilder::new().env(Environment::Prod);
        assert_eq!(builder.env, Environment::Prod);
    }
}
