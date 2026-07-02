use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

use crate::{
    LoggerError,
    config::{LogLevel, LoggerConfig, Rotation},
    error::Result,
};

use crate::layer;

/// Builder 模式
#[derive(Debug, Clone)]
pub struct LoggerBuilder {
    config: LoggerConfig,
}

impl LoggerBuilder {
    pub fn new() -> Self {
        Self {
            config: LoggerConfig::default(),
        }
    }

    pub fn level(mut self, level: LogLevel) -> Self {
        self.config.level = level;
        self
    }

    pub fn console(mut self, enable: bool) -> Self {
        self.config.console.enabled = enable;
        self
    }

    pub fn json(mut self, enable: bool) -> Self {
        self.config.json.enabled = enable;
        self
    }
    pub fn file(mut self, enable: bool) -> Self {
        self.config.file.enabled = enable;
        self
    }
    pub fn directory<S: Into<String>>(mut self, dir: S) -> Self {
        self.config.file.directory = dir.into();
        self
    }
    pub fn filename<S: Into<String>>(mut self, file: S) -> Self {
        self.config.file.filename = file.into();
        self
    }
    pub fn rotation(mut self, rotation: Rotation) -> Self {
        self.config.file.rotation = rotation;
        self
    }

    pub fn config(mut self, cfg: LoggerConfig) -> Self {
        self.config = cfg;

        self
    }

    pub fn build(self) -> LoggerConfig {
        self.config
    }

    pub fn init(self) -> Result<()> {
        crate::init::init(&self.config)
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self {
            config: LoggerConfig::default(),
        }
    }
}
