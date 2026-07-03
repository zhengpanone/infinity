use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 配置错误类型
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum ConfigError {
    /// 文件不存在错误
    #[error("配置文件不存在: {0}")]
    FileNotFound(String),
    /// 文件读取错误
    #[error("无法读取配置文件: {0}")]
    FileReadError(String),

    /// 文件解析错误
    #[error("配置文件解析失败: {0}")]
    ParseError(String),

    /// 环境变量错误
    #[error("环境变量设置错误: {0}")]
    EnvVarError(String),

    /// 配置值缺失错误
    #[error("配置项缺失: {0}")]
    MissingField(String),

    /// 配置值类型错误
    #[error("配置项类型错误: {0}")]
    InvalidType(String),

    /// 配置值验证错误
    #[error("配置项验证失败: {0}")]
    ValidationError(String),

    /// 配置值范围错误
    #[error("配置项值超出范围: {0}")]
    OutOfRange(String),

    /// 配置值格式错误
    #[error("配置项格式错误: {0}")]
    InvalidFormat(String),

    /// 未知错误
    #[error("未知配置错误: {0}")]
    Unknown(String),
}

impl ConfigError {
    /// 文件不存在错误
    pub fn file_not_found(path: impl Into<String>) -> Self {
        Self::FileNotFound(path.into())
    }
    /// 配置值缺失错误
    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    // TODO: 其他错误类型
}

/// 将 std::io::Error 转换为 ConfigError
impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => ConfigError::FileNotFound(err.to_string()),
            _ => ConfigError::FileReadError(err.to_string()),
        }
    }
}

/// 将 serde_json::Error 转换为 ConfigError
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
