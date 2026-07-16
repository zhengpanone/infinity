//! 手机号脱敏。

/// 脱敏手机号:保留前 3 位与后 4 位,中间以 `****` 代替。
///
/// 适用于 11 位中国大陆手机号;可选的 `+86`/`86` 前缀会原样保留。
/// 长度不足 8 位(无法同时保留前 3 后 4)的输入整体替换为 `******`,
/// 避免短号码泄露过多字符。
///
/// # 示例
///
/// ```
/// use infinity_audit::mask::mask_phone;
///
/// assert_eq!(mask_phone("13812345678"), "138****5678");
/// assert_eq!(mask_phone("+8613812345678"), "+86138****5678");
/// assert_eq!(mask_phone("12345"), "******");
/// ```
pub fn mask_phone(phone: &str) -> String {
    let (prefix, digits) = split_country_code(phone.trim());

    if digits.len() < 8 || !digits.chars().all(|c| c.is_ascii_digit()) {
        return "******".to_owned();
    }

    let head = &digits[..3];
    let tail = &digits[digits.len() - 4..];
    format!("{prefix}{head}****{tail}")
}

/// 拆出 `+86`/`86` 国家码前缀,返回(前缀, 剩余号码)。
fn split_country_code(phone: &str) -> (&str, &str) {
    if let Some(rest) = phone.strip_prefix("+86") {
        (&phone[..3], rest)
    } else if phone.len() > 11 && phone.starts_with("86") {
        (&phone[..2], &phone[2..])
    } else {
        ("", phone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_mainland_mobile_number() {
        assert_eq!(mask_phone("13812345678"), "138****5678");
    }

    #[test]
    fn keeps_country_code_prefix() {
        assert_eq!(mask_phone("+8613812345678"), "+86138****5678");
        assert_eq!(mask_phone("8613812345678"), "86138****5678");
    }

    #[test]
    fn short_or_invalid_input_is_fully_masked() {
        assert_eq!(mask_phone("12345"), "******");
        assert_eq!(mask_phone(""), "******");
        assert_eq!(mask_phone("not-a-phone"), "******");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        assert_eq!(mask_phone(" 13812345678 "), "138****5678");
    }
}
