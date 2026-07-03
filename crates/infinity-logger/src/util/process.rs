//! 进程工具。

/// 获取当前进程 ID。
///
/// # Examples
///
/// ```
/// let pid = infinity_logger::util::process::id();
/// assert!(pid > 0);
/// ```
pub fn id() -> u32 {
    std::process::id()
}

/// 获取当前可执行文件名（不含路径）。
///
/// 如果无法获取，返回 `"unknown"`。
pub fn name() -> String {
    std::env::current_exe()
        .ok()
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_id_positive() {
        assert!(id() > 0);
    }

    #[test]
    fn test_process_name_not_empty() {
        assert!(!name().is_empty());
    }
}
