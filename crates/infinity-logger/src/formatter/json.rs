//! JSON 格式化辅助。
//!
//! 提供为 `fmt::Layer` 应用 JSON 格式配置的辅助函数。实际的 JSON Layer
//! 构建位于 [`crate::layer::json`]，此模块用于共享格式化选项。

use crate::config::LoggerConfig;

/// 是否启用美化（pretty）JSON 输出。
///
/// # Examples
///
/// ```
/// use infinity_logger::config::LoggerConfig;
///
/// let config = LoggerConfig::default();
/// assert!(!infinity_logger::formatter::json::is_pretty(&config));
/// ```
pub fn is_pretty(config: &LoggerConfig) -> bool {
    config.json.pretty
}

/// 是否扁平化字段。
pub fn is_flatten(config: &LoggerConfig) -> bool {
    config.json.flatten
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_not_pretty() {
        let config = LoggerConfig::default();
        assert!(!is_pretty(&config));
    }

    #[test]
    fn test_default_flatten() {
        let config = LoggerConfig::default();
        assert!(is_flatten(&config));
    }
}
