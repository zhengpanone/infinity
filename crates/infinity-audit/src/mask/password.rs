//! 密码及机密字段脱敏。

/// 密码脱敏后的固定占位符。
///
/// 无论原文长短一律输出同一占位符,连长度信息都不泄露。
pub const PASSWORD_PLACEHOLDER: &str = "******";

/// 脱敏密码:任何输入都返回 [`PASSWORD_PLACEHOLDER`]。
///
/// 密码(包括其哈希)绝不应出现在审计日志中;本函数存在的意义是让调用方
/// 显式声明"这里有一个密码字段",而非静默丢弃。
///
/// # 示例
///
/// ```
/// use infinity_audit::mask::mask_password;
///
/// assert_eq!(mask_password("hunter2"), "******");
/// assert_eq!(mask_password(""), "******");
/// ```
pub fn mask_password(_password: &str) -> String {
    PASSWORD_PLACEHOLDER.to_owned()
}

/// 判断字段名是否属于机密字段(密码、令牌、密钥等)。
///
/// 供审计中间件与服务在记录字段变更前做兜底检查;命中的字段应使用
/// [`FieldChange::masked`](crate::domain::FieldChange::masked) 记录。
/// 匹配不区分大小写,按子串匹配(如 `user_password`、`apiToken` 均命中)。
pub fn is_secret_field(field: &str) -> bool {
    const SECRET_MARKERS: [&str; 6] = ["password", "passwd", "secret", "token", "api_key", "apikey"];

    let lowered = field.to_ascii_lowercase();
    SECRET_MARKERS
        .iter()
        .any(|marker| lowered.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_password_becomes_fixed_placeholder() {
        assert_eq!(mask_password("hunter2"), PASSWORD_PLACEHOLDER);
        assert_eq!(mask_password(""), PASSWORD_PLACEHOLDER);
        assert_eq!(
            mask_password("a-very-long-password-with-entropy"),
            PASSWORD_PLACEHOLDER
        );
    }

    #[test]
    fn secret_field_detection_is_case_insensitive_substring() {
        assert!(is_secret_field("password"));
        assert!(is_secret_field("user_password_hash"));
        assert!(is_secret_field("apiToken"));
        assert!(is_secret_field("API_KEY"));
        assert!(is_secret_field("client_secret"));
        assert!(!is_secret_field("username"));
        assert!(!is_secret_field("email"));
    }
}
