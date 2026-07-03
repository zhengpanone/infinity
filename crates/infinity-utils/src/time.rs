//! 时间工具。

use chrono::{DateTime, Utc};

/// 当前 UTC 时间。
///
/// # Examples
///
/// ```
/// let _now = infinity_utils::time::now();
/// ```
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// 当前 Unix 毫秒时间戳。
pub fn now_millis() -> i64 {
    Utc::now().timestamp_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_millis_positive() {
        assert!(now_millis() > 0);
    }
}
