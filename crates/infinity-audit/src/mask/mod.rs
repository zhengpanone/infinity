//! 敏感数据脱敏。
//!
//! 审计日志会长期留存并被较多人查看,敏感信息(密码、手机号等)必须在
//! 落库前脱敏。本模块提供各类字段的脱敏函数与机密字段判定。

pub mod password;
pub mod phone;

pub use password::{PASSWORD_PLACEHOLDER, is_secret_field, mask_password};
pub use phone::mask_phone;
