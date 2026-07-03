use crate::error::Result;

/// 可自行校验约束的配置结构体 trait。
pub trait Validate {
    /// 校验配置。
    ///
    /// # Errors
    ///
    /// 校验失败时返回 [`crate::error::ConfigError`]。
    fn validate(&self) -> Result<()>;
}
