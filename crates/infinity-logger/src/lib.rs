//! # infinity-logger
//!
//! 企业级日志组件，为 Infinity Workspace 提供统一的日志基础设施。
//!
//! 基于 `tracing` 构建，支持：
//!
//! - Console
//! - Rolling File
//! - JSON
//! - OpenTelemetry
//! - Axum
//! - gRPC
//! - RequestId
//! - TraceId
//!
//! 推荐初始化：
//!
//! ```no_run
//! use infinity_logger::Logger;
//!
//! fn main() -> Result<(), infinity_logger::LoggerError> {
//!     Logger::builder()
//!         .init()?;
//!
//!     tracing::info!("Logger initialized");
//!     Ok(())
//! }
//! ```

pub mod builder;
pub mod config;
pub mod error;
pub mod logger;

mod init;

#[cfg(feature = "console")]
pub mod formatter;

pub mod layer;

#[cfg(any(feature = "axum", feature = "grpc"))]
pub mod middleware;

pub mod macros;
pub mod util;

pub use builder::LoggerBuilder;
pub use error::{LoggerError, Result};
pub use logger::Logger;

/// 当前 crate版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 使用默认配置初始化 Logger。
///
/// 等价于：
///
/// ```no_run
/// infinity_logger::Logger::builder().init()?;
/// # Ok::<(), infinity_logger::LoggerError>(())
/// ```
#[inline]
pub fn init() -> Result<()> {
    Logger::builder().init()
}
