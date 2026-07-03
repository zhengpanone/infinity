use std::path::Path;

use once_cell::sync::OnceCell;

use crate::{
    config::AppConfig, environment::Environment, error::ConfigError, error::Result, loader,
    traits::Validate,
};

static GLOBAL: OnceCell<AppConfig> = OnceCell::new();

/// 进程级配置管理器。
pub struct ConfigManager;

impl ConfigManager {
    /// 使用已加载好的配置值初始化全局配置。
    pub fn init(config: AppConfig) -> Result<()> {
        config.validate()?;
        GLOBAL
            .set(config)
            .map_err(|_| ConfigError::Unknown("configuration has already been initialized".into()))
    }

    /// 从目录加载 `application.toml` 和 `application-<env>.toml`。
    pub fn init_from_dir(dir: impl AsRef<Path>) -> Result<&'static AppConfig> {
        Self::init_from_dir_with_env(dir, Environment::detect())
    }

    /// 按显式环境从目录加载。
    pub fn init_from_dir_with_env(
        dir: impl AsRef<Path>,
        env: Environment,
    ) -> Result<&'static AppConfig> {
        let config = Self::load_from_dir_with_env(dir, env)?;
        Self::init(config)?;
        Self::current()
    }

    /// 仅加载配置，不写入全局单例。
    pub fn load_from_dir_with_env(dir: impl AsRef<Path>, env: Environment) -> Result<AppConfig> {
        let dir = dir.as_ref();
        let base = dir.join("application.toml");
        let overlay = dir.join(format!("application-{}.toml", env.as_str()));
        let config: AppConfig = loader::load_layered_toml(&base, Some(&overlay))?;
        config.validate()?;
        Ok(config)
    }

    /// 返回已初始化的全局配置。
    pub fn current() -> Result<&'static AppConfig> {
        GLOBAL
            .get()
            .ok_or_else(|| ConfigError::Unknown("configuration has not been initialized".into()))
    }

    /// 返回已初始化的全局配置（若存在）。
    pub fn try_current() -> Option<&'static AppConfig> {
        GLOBAL.get()
    }
}
