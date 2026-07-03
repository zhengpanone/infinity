use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 配置 crate 统一使用的 Result 类型。
pub type Result<T> = std::result::Result<T, ConfigError>;

/// 配置加载、解析或校验过程中返回的错误类型。
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ConfigError {
    /// 配置文件不存在。
    #[error("configuration file not found: {0}")]
    FileNotFound(String),

    /// 配置文件读取失败。
    #[error("failed to read configuration file: {0}")]
    FileReadError(String),

    /// 配置文件解析失败。
    #[error("failed to parse configuration file: {0}")]
    ParseError(String),

    /// 环境变量无效。
    #[error("invalid environment variable: {0}")]
    EnvVarError(String),

    /// 配置项缺失。
    #[error("missing configuration field: {0}")]
    MissingField(String),

    /// 配置项类型错误。
    #[error("invalid configuration field type: {0}")]
    InvalidType(String),

    /// 配置项校验失败。
    #[error("configuration validation failed: {0}")]
    ValidationError(String),

    /// 配置项值超出范围。
    #[error("configuration value out of range: {0}")]
    OutOfRange(String),

    /// 文件或值格式不受支持。
    #[error("invalid configuration format: {0}")]
    InvalidFormat(String),

    /// 未知配置错误。
    #[error("unknown configuration error: {0}")]
    Unknown(String),

    /// 不支持的操作。
    #[error("unsupported feature: {0}")]
    Unsupported(String),

    /// 普通消息错误。
    #[error("{0}")]
    Message(String),
}

impl ConfigError {
    /// 构造文件不存在错误。
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound(path.into())
    }

    /// 构造配置项缺失错误。
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    /// 构造校验失败错误。
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    /// 构造环境变量错误。
    pub fn env_var(msg: impl Into<String>) -> Self {
        Self::EnvVarError(msg.into())
    }

    /// 构造无效配置错误。
    pub fn invalid(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    /// 构造不支持功能错误。
    pub fn unsupported(msg: impl Into<String>) -> Self {
        Self::Unsupported(msg.into())
    }

    /// 构造普通消息错误。
    pub fn message(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => ConfigError::FileNotFound(err.to_string()),
            _ => ConfigError::FileReadError(err.to_string()),
        }
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::ParseError(err.to_string())
    }
}
