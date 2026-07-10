use thiserror::Error;

/// 配置 crate 统一使用的 Result 类型。
pub type Result<T> = std::result::Result<T, ConfigError>;

/// 配置加载、解析或校验过程中返回的错误类型。
#[derive(Debug, Error)]
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

/// 在工作区边界把 [`ConfigError`] 转换为统一的 [`InfinityError`]。
///
/// 大多数配置失败都属于配置加载/解析/校验，归入 [`ErrorKind::Config`]；
/// `Unsupported` 与 `Message` 保留各自语义。原始 `ConfigError` 会作为
/// source 保留，形成完整错误链。
impl From<ConfigError> for infinity_error::InfinityError {
    fn from(err: ConfigError) -> Self {
        use infinity_error::ErrorKind;

        let kind = match &err {
            ConfigError::Unsupported(_) => ErrorKind::Unsupported,
            ConfigError::Message(_) => ErrorKind::Message,
            _ => ErrorKind::Config,
        };
        infinity_error::InfinityError::with_source(kind, err.to_string(), err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use infinity_error::{ErrorKind, InfinityError};
    use std::error::Error;

    #[test]
    fn config_error_maps_to_config_kind_and_preserves_source() {
        let err: InfinityError = ConfigError::missing_field("server.port").into();

        assert_eq!(err.kind(), ErrorKind::Config);
        assert_eq!(err.status_code(), 500);
        // 原始 ConfigError 作为 source 保留
        let source = err.source().expect("source 应被保留");
        assert!(source.to_string().contains("server.port"));
    }

    #[test]
    fn unsupported_and_message_keep_their_kind() {
        let unsupported: InfinityError = ConfigError::unsupported("hot reload").into();
        assert_eq!(unsupported.kind(), ErrorKind::Unsupported);

        let message: InfinityError = ConfigError::message("boom").into();
        assert_eq!(message.kind(), ErrorKind::Message);
    }
}
