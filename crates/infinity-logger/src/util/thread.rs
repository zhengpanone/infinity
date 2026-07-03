//! 线程工具。

/// 获取当前线程名。
///
/// 未命名线程返回 `"unnamed"`。
///
/// # Examples
///
/// ```
/// let name = infinity_logger::util::thread::name();
/// assert!(!name.is_empty());
/// ```
pub fn name() -> String {
    std::thread::current()
        .name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unnamed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_thread() {
        let handle = std::thread::Builder::new()
            .name("worker-1".to_string())
            .spawn(|| name())
            .expect("spawn thread");
        assert_eq!(handle.join().expect("join"), "worker-1");
    }
}
