//! 统一错误类型。
//!
//! 所有公开 API 返回 [`Result`]，错误统一转换为 [`LoggerError`]，
//! 禁止向外暴露第三方错误类型。

use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, LoggerError>;

#[derive(Debug, Error)]
pub enum LoggerError {
    /// IO 错误
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    /// TOML 解析失败
    #[error("toml parse error: {0}")]
    TomlDe(#[from] toml::de::Error),

    /// TOML 序列化失败
    #[error("toml serialize error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    /// tracing subscriber 已初始化
    #[error("logger has already been initialized")]
    AlreadyInitialized,

    /// 配置错误
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    /// 不支持的功能
    #[error("unsupported feature: {0}")]
    Unsupported(String),

    /// 其它错误
    #[error("{0}")]
    Message(String),
}

impl LoggerError {
    /// 快速创建配置错误
    pub fn invalid(msg: impl Into<String>) -> Self {
        Self::InvalidConfig(msg.into())
    }

    /// 快速创建普通错误
    pub fn message(msg: impl Into<String>) -> Self {
        Self::Message(msg.into())
    }
}

/// 在工作区边界把 [`LoggerError`] 转换为统一的 [`InfinityError`]。
///
/// 日志相关失败归入 [`ErrorKind::Logger`]；`Unsupported` 与 `Message`
/// 保留各自语义。原始 `LoggerError` 会作为 source 保留，形成完整错误链。
impl From<LoggerError> for infinity_error::InfinityError {
    fn from(err: LoggerError) -> Self {
        use infinity_error::ErrorKind;

        let kind = match &err {
            LoggerError::Unsupported(_) => ErrorKind::Unsupported,
            LoggerError::Message(_) => ErrorKind::Message,
            _ => ErrorKind::Logger,
        };
        infinity_error::InfinityError::with_source(kind, err.to_string(), err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_config_display() {
        let err = LoggerError::invalid("bad path");
        assert_eq!(err.to_string(), "invalid configuration: bad path");
    }

    #[test]
    fn test_message_display() {
        let err = LoggerError::message("boom");
        assert_eq!(err.to_string(), "boom");
    }

    #[test]
    fn test_already_initialized_display() {
        let err = LoggerError::AlreadyInitialized;
        assert_eq!(err.to_string(), "logger has already been initialized");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "missing");
        let err: LoggerError = io_err.into();
        assert!(matches!(err, LoggerError::Io(_)));
    }

    #[test]
    fn logger_error_maps_to_logger_kind_and_preserves_source() {
        use infinity_error::{ErrorKind, InfinityError};
        use std::error::Error;

        let err: InfinityError = LoggerError::AlreadyInitialized.into();
        assert_eq!(err.kind(), ErrorKind::Logger);
        assert_eq!(err.status_code(), 500);
        assert!(err.source().is_some());
    }

    #[test]
    fn logger_unsupported_and_message_keep_their_kind() {
        use infinity_error::{ErrorKind, InfinityError};

        let unsupported: InfinityError = LoggerError::Unsupported("otel".into()).into();
        assert_eq!(unsupported.kind(), ErrorKind::Unsupported);

        let message: InfinityError = LoggerError::message("boom").into();
        assert_eq!(message.kind(), ErrorKind::Message);
    }
}
