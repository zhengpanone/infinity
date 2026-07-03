//! 主机名工具。

/// 获取当前主机名。
///
/// 如果无法获取（例如系统调用失败或包含非法 UTF-8），返回 `"unknown"`。
///
/// # Examples
///
/// ```
/// let host = infinity_logger::util::hostname::get();
/// assert!(!host.is_empty());
/// ```
pub fn get() -> String {
    hostname::get()
        .ok()
        .and_then(|h| h.into_string().ok())
        .unwrap_or_else(|| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostname_not_empty() {
        let host = get();
        assert!(!host.is_empty());
    }
}
