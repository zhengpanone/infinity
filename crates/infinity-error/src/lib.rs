//! Infinity 工作区共享错误基础设施。
//!
//! `infinity-error` 有意保持轻量、低依赖。它提供一个稳定的错误封装，
//! 让应用 crate 和基础设施 crate 可以共享错误分类，同时避免 crate 之间产生循环依赖。

use std::{fmt, io};

use thiserror::Error;

/// 工作区统一使用的 `Result` 类型。
pub type Result<T> = std::result::Result<T, InfinityError>;

/// 工作区错误的稳定分类。
///
/// 标记为 `#[non_exhaustive]`：在 `1.0` 之前可能新增分类,下游 `match`
/// 请始终保留 `_` 分支。参见设计文档 §10。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// 配置加载、解析或校验失败。
    Config,
    /// 日志初始化或日志配置失败。
    Logger,
    /// 数据库访问或连接管理失败。
    Database,
    /// 缓存访问或连接管理失败。
    Cache,
    /// 认证或授权失败。
    Auth,
    /// HTTP 或 Web 框架处理失败。
    Web,
    /// 输入未通过校验。
    Validation,
    /// 请求的资源不存在。
    NotFound,
    /// 请求的操作与当前状态冲突。
    Conflict,
    /// 请求缺少有效凭证。
    Unauthorized,
    /// 调用方已认证，但没有权限执行该操作。
    Forbidden,
    /// 文件系统或进程 IO 失败。
    Io,
    /// 结构化数据解析失败。
    Parse,
    /// 功能未启用或暂未实现。
    Unsupported,
    /// 非预期的内部错误。
    Internal,
    /// 通用消息错误。
    Message,
}

impl ErrorKind {
    /// 所有错误分类，按声明顺序排列。
    ///
    /// 便于遍历（例如预生成 metrics label 集合）或在测试中做穷尽校验。
    pub const ALL: [ErrorKind; 16] = [
        Self::Config,
        Self::Logger,
        Self::Database,
        Self::Cache,
        Self::Auth,
        Self::Web,
        Self::Validation,
        Self::NotFound,
        Self::Conflict,
        Self::Unauthorized,
        Self::Forbidden,
        Self::Io,
        Self::Parse,
        Self::Unsupported,
        Self::Internal,
        Self::Message,
    ];

    /// 返回稳定的小写错误码，适合日志字段和 API 响应使用。
    pub const fn code(self) -> &'static str {
        match self {
            Self::Config => "config",
            Self::Logger => "logger",
            Self::Database => "database",
            Self::Cache => "cache",
            Self::Auth => "auth",
            Self::Web => "web",
            Self::Validation => "validation",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::Unauthorized => "unauthorized",
            Self::Forbidden => "forbidden",
            Self::Io => "io",
            Self::Parse => "parse",
            Self::Unsupported => "unsupported",
            Self::Internal => "internal",
            Self::Message => "message",
        }
    }

    /// 返回该错误分类通常对应的 HTTP 状态码。
    pub const fn status_code(self) -> u16 {
        match self {
            Self::Validation | Self::Parse => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::Unsupported => 501,
            Self::Config
            | Self::Logger
            | Self::Database
            | Self::Cache
            | Self::Auth
            | Self::Web
            | Self::Io
            | Self::Internal
            | Self::Message => 500,
        }
    }

    /// 从稳定错误码解析回分类；未知错误码返回 `None`。
    ///
    /// 与 [`code`](Self::code) 互为逆操作，可用于反序列化或从日志字段还原分类。
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.code() == code)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// Infinity crate 和应用共享的错误类型。
///
/// 标记为 `#[non_exhaustive]`：在 `1.0` 之前可能新增变体,下游 `match`
/// 请始终保留 `_` 分支。参见设计文档 §10。
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InfinityError {
    /// 配置加载、解析或校验失败。
    #[error("configuration error: {message}")]
    Config { message: String },

    /// 日志初始化或日志配置失败。
    #[error("logger error: {message}")]
    Logger { message: String },

    /// 数据库访问或连接管理失败。
    #[error("database error: {message}")]
    Database { message: String },

    /// 缓存访问或连接管理失败。
    #[error("cache error: {message}")]
    Cache { message: String },

    /// 认证或授权失败。
    #[error("auth error: {message}")]
    Auth { message: String },

    /// HTTP 或 Web 框架处理失败。
    #[error("web error: {message}")]
    Web { message: String },

    /// 输入未通过校验。
    #[error("validation error{field}: {message}", field = format_field(field))]
    Validation {
        /// 校验失败的字段或路径，可选。
        field: Option<String>,
        /// 面向人的校验错误信息。
        message: String,
    },

    /// 请求的资源不存在。
    #[error("not found: {resource}")]
    NotFound { resource: String },

    /// 请求的操作与当前状态冲突。
    #[error("conflict: {resource}")]
    Conflict { resource: String },

    /// 请求缺少有效凭证。
    #[error("unauthorized")]
    Unauthorized,

    /// 调用方已认证，但没有权限执行该操作。
    #[error("forbidden")]
    Forbidden,

    /// 文件系统或进程 IO 失败。
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// 结构化数据解析失败。
    #[error("{format} parse error: {message}")]
    Parse {
        format: &'static str,
        message: String,
    },

    /// 功能未启用或暂未实现。
    #[error("unsupported feature: {feature}")]
    Unsupported { feature: String },

    /// 非预期的内部错误。
    #[error("internal error: {message}")]
    Internal { message: String },

    /// 通用消息错误。
    #[error("{0}")]
    Message(String),

    /// 带上下文消息且保留底层来源错误的封装。
    ///
    /// 由 [`ResultExt`] 和 [`InfinityError::with_source`] 生成，
    /// 底层错误可通过 [`std::error::Error::source`] 访问，形成完整错误链。
    #[error("{message}")]
    Contextual {
        /// 该错误的稳定分类。
        kind: ErrorKind,
        /// 面向人的上下文消息。
        message: String,
        /// 底层来源错误。
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

impl InfinityError {
    /// 创建配置错误。
    pub fn config(message: impl Into<String>) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// 创建日志错误。
    pub fn logger(message: impl Into<String>) -> Self {
        Self::Logger {
            message: message.into(),
        }
    }

    /// 创建数据库错误。
    pub fn database(message: impl Into<String>) -> Self {
        Self::Database {
            message: message.into(),
        }
    }

    /// 创建缓存错误。
    pub fn cache(message: impl Into<String>) -> Self {
        Self::Cache {
            message: message.into(),
        }
    }

    /// 创建认证或授权错误。
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Auth {
            message: message.into(),
        }
    }

    /// 创建 Web 错误。
    pub fn web(message: impl Into<String>) -> Self {
        Self::Web {
            message: message.into(),
        }
    }

    /// 创建不带字段路径的校验错误。
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            field: None,
            message: message.into(),
        }
    }

    /// 创建带字段或路径的校验错误。
    pub fn validation_field(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: Some(field.into()),
            message: message.into(),
        }
    }

    /// 创建资源不存在错误。
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// 创建状态冲突错误。
    pub fn conflict(resource: impl Into<String>) -> Self {
        Self::Conflict {
            resource: resource.into(),
        }
    }

    /// 创建不支持功能错误。
    pub fn unsupported(feature: impl Into<String>) -> Self {
        Self::Unsupported {
            feature: feature.into(),
        }
    }

    /// 创建内部错误。
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }

    /// 创建通用消息错误。
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }

    /// 创建解析错误，并携带稳定的格式标签。
    pub fn parse(format: &'static str, message: impl Into<String>) -> Self {
        Self::Parse {
            format,
            message: message.into(),
        }
    }

    /// 用给定分类和消息构造错误。
    ///
    /// 主要供在 crate 边界统一转换第三方错误使用（见 [`ResultExt`]）。
    /// 注意几个边界情况：
    ///
    /// - [`ErrorKind::Unauthorized`] 与 [`ErrorKind::Forbidden`] 是无字段变体，会忽略 `message`。
    /// - [`ErrorKind::Io`] 会用 [`io::Error::other`] 把消息包装成 IO 错误。
    /// - [`ErrorKind::Parse`] 无法从消息推断格式，格式标签固定为 `"unknown"`。
    pub fn with_kind(kind: ErrorKind, message: impl Into<String>) -> Self {
        let message = message.into();
        match kind {
            ErrorKind::Config => Self::Config { message },
            ErrorKind::Logger => Self::Logger { message },
            ErrorKind::Database => Self::Database { message },
            ErrorKind::Cache => Self::Cache { message },
            ErrorKind::Auth => Self::Auth { message },
            ErrorKind::Web => Self::Web { message },
            ErrorKind::Validation => Self::Validation {
                field: None,
                message,
            },
            ErrorKind::NotFound => Self::NotFound { resource: message },
            ErrorKind::Conflict => Self::Conflict { resource: message },
            ErrorKind::Unauthorized => Self::Unauthorized,
            ErrorKind::Forbidden => Self::Forbidden,
            ErrorKind::Io => Self::Io(io::Error::other(message)),
            ErrorKind::Parse => Self::Parse {
                format: "unknown",
                message,
            },
            ErrorKind::Unsupported => Self::Unsupported { feature: message },
            ErrorKind::Internal => Self::Internal { message },
            ErrorKind::Message => Self::Message(message),
        }
    }

    /// 用给定分类、上下文消息和底层来源错误构造错误。
    ///
    /// 与 [`with_kind`](Self::with_kind) 不同，来源错误会被保留，
    /// 可通过 [`std::error::Error::source`] 访问，形成完整错误链。
    pub fn with_source(
        kind: ErrorKind,
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Contextual {
            kind,
            message: message.into(),
            source: Box::new(source),
        }
    }

    /// 返回该错误对应的稳定分类。
    pub const fn kind(&self) -> ErrorKind {
        match self {
            Self::Config { .. } => ErrorKind::Config,
            Self::Logger { .. } => ErrorKind::Logger,
            Self::Database { .. } => ErrorKind::Database,
            Self::Cache { .. } => ErrorKind::Cache,
            Self::Auth { .. } => ErrorKind::Auth,
            Self::Web { .. } => ErrorKind::Web,
            Self::Validation { .. } => ErrorKind::Validation,
            Self::NotFound { .. } => ErrorKind::NotFound,
            Self::Conflict { .. } => ErrorKind::Conflict,
            Self::Unauthorized => ErrorKind::Unauthorized,
            Self::Forbidden => ErrorKind::Forbidden,
            Self::Io(_) => ErrorKind::Io,
            Self::Parse { .. } => ErrorKind::Parse,
            Self::Unsupported { .. } => ErrorKind::Unsupported,
            Self::Internal { .. } => ErrorKind::Internal,
            Self::Message(_) => ErrorKind::Message,
            Self::Contextual { kind, .. } => *kind,
        }
    }

    /// 返回稳定的小写错误码，适合日志字段和 API 响应使用。
    pub const fn code(&self) -> &'static str {
        self.kind().code()
    }

    /// 返回该错误通常对应的 HTTP 状态码。
    pub const fn status_code(&self) -> u16 {
        self.kind().status_code()
    }

    /// 当错误通常由调用方输入导致时，返回 `true`。
    pub const fn is_client_error(&self) -> bool {
        matches!(
            self.kind(),
            ErrorKind::Validation
                | ErrorKind::NotFound
                | ErrorKind::Conflict
                | ErrorKind::Unauthorized
                | ErrorKind::Forbidden
                | ErrorKind::Parse
        )
    }

    /// 当错误通常代表服务端失败时，返回 `true`。
    pub const fn is_server_error(&self) -> bool {
        !self.is_client_error()
    }
}

impl From<String> for InfinityError {
    fn from(message: String) -> Self {
        Self::Message(message)
    }
}

impl From<&str> for InfinityError {
    fn from(message: &str) -> Self {
        Self::Message(message.to_owned())
    }
}

/// 立即以指定分类和格式化消息从当前函数返回 [`InfinityError`]。
///
/// 消息部分接受 [`format!`] 风格参数。生成的错误会经 [`Into`] 转换，
/// 因此可用于任意返回 `Result<_, E>`（`E: From<InfinityError>`）的函数。
///
/// # 示例
///
/// ```
/// use infinity_error::{bail, ErrorKind, Result};
///
/// fn check(name: &str) -> Result<()> {
///     if name.is_empty() {
///         bail!(ErrorKind::Validation, "name 不能为空");
///     }
///     Ok(())
/// }
///
/// assert_eq!(check("").unwrap_err().kind(), ErrorKind::Validation);
/// ```
#[macro_export]
macro_rules! bail {
    ($kind:expr, $($arg:tt)*) => {
        return ::core::result::Result::Err(::core::convert::Into::into(
            $crate::InfinityError::with_kind($kind, ::std::format!($($arg)*)),
        ))
    };
}

/// 当条件为假时，以指定分类和格式化消息从当前函数返回错误。
///
/// 类似 `assert!`，但失败时返回 [`InfinityError`] 而非 panic。
///
/// # 示例
///
/// ```
/// use infinity_error::{ensure, ErrorKind, Result};
///
/// fn check(age: i32) -> Result<()> {
///     ensure!(age >= 0, ErrorKind::Validation, "age 必须非负，实际为 {age}");
///     Ok(())
/// }
///
/// assert!(check(-1).is_err());
/// assert!(check(0).is_ok());
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $kind:expr, $($arg:tt)*) => {
        if !($cond) {
            $crate::bail!($kind, $($arg)*);
        }
    };
}

/// 在 crate 边界把任意 `Result` 转换为工作区的 [`Result`]。
///
/// 相比手写 `.map_err(|e| InfinityError::database(e.to_string()))`，`ResultExt`
/// 让下游 crate 一行完成转换，并**保留底层错误的类型化来源**（通过
/// [`std::error::Error::source`] 可访问完整错误链），生成的错误分类为传入的
/// [`ErrorKind`]。
///
/// # 示例
///
/// ```
/// use std::error::Error;
/// use infinity_error::{ErrorKind, Result, ResultExt};
///
/// fn load(raw: &str) -> Result<u32> {
///     raw.parse::<u32>().context(ErrorKind::Parse, "解析端口")
/// }
///
/// assert_eq!(load("42").unwrap(), 42);
///
/// let err = load("abc").unwrap_err();
/// assert_eq!(err.kind(), ErrorKind::Parse);
/// assert_eq!(err.to_string(), "解析端口");
/// // 底层 ParseIntError 作为 source 保留下来
/// assert!(err.source().is_some());
/// ```
pub trait ResultExt<T> {
    /// 出错时用指定分类和上下文消息包装错误，并保留底层来源。
    fn context(self, kind: ErrorKind, message: impl Into<String>) -> Result<T>;

    /// 与 [`ResultExt::context`] 相同，但消息按需惰性构造，仅在出错时求值。
    fn with_context<C, F>(self, kind: ErrorKind, message: F) -> Result<T>
    where
        C: Into<String>,
        F: FnOnce() -> C;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn context(self, kind: ErrorKind, message: impl Into<String>) -> Result<T> {
        self.map_err(|err| InfinityError::with_source(kind, message, err))
    }

    fn with_context<C, F>(self, kind: ErrorKind, message: F) -> Result<T>
    where
        C: Into<String>,
        F: FnOnce() -> C,
    {
        self.map_err(|err| InfinityError::with_source(kind, message(), err))
    }
}

fn format_field(field: &Option<String>) -> String {
    match field {
        Some(field) => format!(" in {field}"),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn constructors_assign_expected_kinds() {
        assert_eq!(InfinityError::config("missing").kind(), ErrorKind::Config);
        assert_eq!(InfinityError::logger("init").kind(), ErrorKind::Logger);
        assert_eq!(
            InfinityError::database("connect").kind(),
            ErrorKind::Database
        );
        assert_eq!(InfinityError::cache("redis").kind(), ErrorKind::Cache);
        assert_eq!(InfinityError::auth("token").kind(), ErrorKind::Auth);
        assert_eq!(InfinityError::web("route").kind(), ErrorKind::Web);
    }

    #[test]
    fn kind_codes_are_stable() {
        assert_eq!(ErrorKind::Validation.code(), "validation");
        assert_eq!(ErrorKind::NotFound.code(), "not_found");
        assert_eq!(InfinityError::unsupported("otel").code(), "unsupported");
    }

    #[test]
    fn status_codes_follow_http_defaults() {
        assert_eq!(InfinityError::validation("bad").status_code(), 400);
        assert_eq!(InfinityError::Unauthorized.status_code(), 401);
        assert_eq!(InfinityError::Forbidden.status_code(), 403);
        assert_eq!(InfinityError::not_found("user").status_code(), 404);
        assert_eq!(InfinityError::conflict("user").status_code(), 409);
        assert_eq!(InfinityError::unsupported("feature").status_code(), 501);
        assert_eq!(InfinityError::database("down").status_code(), 500);
    }

    #[test]
    fn validation_field_display_includes_field_name() {
        let err = InfinityError::validation_field("database.url", "must not be empty");
        assert_eq!(
            err.to_string(),
            "validation error in database.url: must not be empty"
        );
    }

    #[test]
    fn client_and_server_classification() {
        assert!(InfinityError::not_found("user").is_client_error());
        assert!(!InfinityError::not_found("user").is_server_error());
        assert!(InfinityError::internal("boom").is_server_error());
    }

    #[test]
    fn io_errors_are_preserved_as_source_errors() {
        let err = io::Error::new(io::ErrorKind::NotFound, "missing");
        let err: InfinityError = err.into();

        assert_eq!(err.kind(), ErrorKind::Io);
        assert_eq!(err.status_code(), 500);
        assert!(err.source().is_some());
    }

    #[test]
    fn strings_convert_to_message_errors() {
        let owned: InfinityError = String::from("boom").into();
        let borrowed: InfinityError = "boom".into();

        assert_eq!(owned.kind(), ErrorKind::Message);
        assert_eq!(borrowed.to_string(), "boom");
    }

    #[test]
    fn with_kind_maps_every_kind_to_matching_error() {
        for kind in [
            ErrorKind::Config,
            ErrorKind::Logger,
            ErrorKind::Database,
            ErrorKind::Cache,
            ErrorKind::Auth,
            ErrorKind::Web,
            ErrorKind::Validation,
            ErrorKind::NotFound,
            ErrorKind::Conflict,
            ErrorKind::Unauthorized,
            ErrorKind::Forbidden,
            ErrorKind::Io,
            ErrorKind::Parse,
            ErrorKind::Unsupported,
            ErrorKind::Internal,
            ErrorKind::Message,
        ] {
            assert_eq!(InfinityError::with_kind(kind, "boom").kind(), kind);
        }
    }

    #[test]
    fn with_kind_io_preserves_source() {
        let err = InfinityError::with_kind(ErrorKind::Io, "disk full");
        assert_eq!(err.kind(), ErrorKind::Io);
        assert!(err.source().is_some());
        assert_eq!(err.to_string(), "io error: disk full");
    }

    #[test]
    fn result_ext_context_wraps_and_preserves_source() {
        let parsed: Result<u32> = "abc".parse::<u32>().context(ErrorKind::Parse, "解析端口");
        let err = parsed.unwrap_err();

        assert_eq!(err.kind(), ErrorKind::Parse);
        assert_eq!(err.status_code(), 400);
        assert_eq!(err.to_string(), "解析端口");
        // 底层 ParseIntError 作为 source 保留，形成完整错误链
        let source = err.source().expect("source 应被保留");
        assert_eq!(source.to_string(), "invalid digit found in string");
    }

    #[test]
    fn result_ext_passes_through_ok() {
        let ok: Result<u32> = "42".parse::<u32>().context(ErrorKind::Parse, "解析端口");
        assert_eq!(ok.unwrap(), 42);
    }

    #[test]
    fn result_ext_with_context_is_lazy() {
        let mut called = false;
        let ok: std::result::Result<u32, std::io::Error> = Ok(7);
        let _ = ok.with_context(ErrorKind::Internal, || {
            called = true;
            "never built"
        });
        assert!(!called, "错误消息不应在 Ok 时被求值");
    }

    #[test]
    fn with_source_preserves_kind_and_chain() {
        let io = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let err = InfinityError::with_source(ErrorKind::Database, "打开连接失败", io);

        assert_eq!(err.kind(), ErrorKind::Database);
        assert_eq!(err.to_string(), "打开连接失败");
        assert_eq!(err.source().unwrap().to_string(), "denied");
    }

    #[test]
    fn error_kind_all_matches_code_round_trip() {
        for kind in ErrorKind::ALL {
            assert_eq!(ErrorKind::from_code(kind.code()), Some(kind));
        }
        assert_eq!(ErrorKind::from_code("does-not-exist"), None);
    }

    #[test]
    fn bail_returns_error_with_kind() {
        fn check(name: &str) -> Result<()> {
            if name.is_empty() {
                bail!(ErrorKind::Validation, "name 不能为空");
            }
            Ok(())
        }

        assert!(check("ok").is_ok());
        let err = check("").unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Validation);
        assert_eq!(err.to_string(), "validation error: name 不能为空");
    }

    #[test]
    fn ensure_returns_error_when_condition_false() {
        fn check(age: i32) -> Result<i32> {
            ensure!(
                age >= 0,
                ErrorKind::Validation,
                "age 必须非负，实际为 {age}"
            );
            Ok(age)
        }

        assert_eq!(check(5).unwrap(), 5);
        assert_eq!(check(-1).unwrap_err().kind(), ErrorKind::Validation);
    }
}
