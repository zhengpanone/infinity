//! 时间格式化。
//!
//! 提供统一的时间戳格式，供各 Layer 复用。默认采用 RFC 3339 格式并使用本地时区。

use tracing_subscriber::fmt::time::{FormatTime, LocalTime, UtcTime};

/// 本地时区 RFC 3339 时间格式器。
///
/// # Examples
///
/// ```
/// let _timer = infinity_logger::formatter::time::local_rfc3339();
/// ```
pub fn local_rfc3339() -> impl FormatTime {
    LocalTime::rfc_3339()
}

/// UTC 时区 RFC 3339 时间格式器。
pub fn utc_rfc3339() -> impl FormatTime {
    UtcTime::rfc_3339()
}
