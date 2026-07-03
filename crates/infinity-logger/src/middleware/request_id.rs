//! Request ID 支持。
//!
//! 提供请求 ID 的生成与常量定义，供 HTTP / gRPC 中间件复用。
//! Request ID 用于将同一请求产生的多条日志关联起来。

use uuid::Uuid;

/// 标准 Request ID Header 名称。
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// 生成一个新的 Request ID（UUID v4）。
///
/// # Examples
///
/// ```
/// let id = infinity_logger::middleware::request_id::generate();
/// assert_eq!(id.len(), 36); // UUID v4 hyphenated
/// ```
pub fn generate() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_unique() {
        let a = generate();
        let b = generate();
        assert_ne!(a, b);
        assert_eq!(a.len(), 36);
    }

    #[test]
    fn test_header_constant() {
        assert_eq!(REQUEST_ID_HEADER, "x-request-id");
    }
}
