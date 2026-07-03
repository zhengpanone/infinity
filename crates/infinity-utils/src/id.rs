//! ID 生成工具。

use uuid::Uuid;

/// 生成一个基于时间有序的 UUID v7 字符串。
///
/// UUID v7 内含时间戳前缀，天然可按生成顺序排序，适合作为数据库主键。
///
/// # Examples
///
/// ```
/// let id = infinity_utils::id::uuid_v7();
/// assert_eq!(id.len(), 36);
/// ```
pub fn uuid_v7() -> String {
    Uuid::now_v7().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_v7_len() {
        assert_eq!(uuid_v7().len(), 36);
    }

    #[test]
    fn test_uuid_v7_unique() {
        assert_ne!(uuid_v7(), uuid_v7());
    }
}
