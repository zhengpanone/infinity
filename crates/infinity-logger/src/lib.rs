//! # logger
//!
//! 企业级日志组件。
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
//! use logger::Logger;
//!
//! fn main() -> Result<(), logger::LoggerError> {
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
pub mod init;
pub mod subscriber;

pub mod logger;

#[cfg(feature = "console")]
pub mod formatter;

#[cfg(feature = "console")]
pub mod layer;

#[cfg(feature = "axum")]
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
/// logger::Logger::builder().init()?;
/// # Ok::<(), logger::LoggerError>(())
/// ```
#[inline]
pub fn init() -> Result<()> {
    Logger::builder().init()
}
