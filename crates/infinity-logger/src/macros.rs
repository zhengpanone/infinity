//! 自定义日志宏。
//!
//! 目前直接复用 `tracing` 提供的日志宏（`info!`、`debug!` 等），
//! 本模块提供少量便捷宏，未来可扩展带上下文的宏。

/// 记录一条带耗时的信息日志。
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// # fn main() {
/// infinity_logger::log_elapsed!("db_query", Duration::from_millis(12));
/// # }
/// ```
#[macro_export]
macro_rules! log_elapsed {
    ($op:expr, $elapsed:expr) => {
        $crate::macros::__private_tracing::info!(
            operation = $op,
            elapsed_ms = $elapsed.as_millis() as u64,
            "operation completed"
        );
    };
}

#[doc(hidden)]
pub use tracing as __private_tracing;
