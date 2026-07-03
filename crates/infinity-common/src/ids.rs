//! 共享领域标识类型。
//!
//! 使用 newtype 封装字符串 ID，避免不同实体的 ID 在类型层面被混用。
//! ID 生成委托给 [`infinity_utils::id`]，体现 common -> utils 的单向依赖。

use serde::{Deserialize, Serialize};

/// 用户 ID。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    /// 基于新生成的 UUID v7 创建用户 ID。
    pub fn generate() -> Self {
        Self(infinity_utils::id::uuid_v7())
    }

    /// 从已有字符串构造。
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 返回内部字符串引用。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 租户 ID。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(String);

impl TenantId {
    /// 基于新生成的 UUID v7 创建租户 ID。
    pub fn generate() -> Self {
        Self(infinity_utils::id::uuid_v7())
    }

    /// 从已有字符串构造。
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// 返回内部字符串引用。
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_generate_unique() {
        assert_ne!(UserId::generate(), UserId::generate());
    }

    #[test]
    fn test_user_id_roundtrip() {
        let id = UserId::new("u-123");
        assert_eq!(id.as_str(), "u-123");
    }

    #[test]
    fn test_tenant_id_new() {
        let id = TenantId::new("t-1");
        assert_eq!(id.as_str(), "t-1");
    }
}
