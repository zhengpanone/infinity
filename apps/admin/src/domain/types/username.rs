use std::fmt::{self, Display, Formatter};

use infinity_error::{InfinityError, Result};
use serde::{Deserialize, Serialize};

/// 用户名值对象
/// 封装用户名验证规则：
/// - 长度 3-20 个字符
/// - 只允许字母、数字、下划线
/// - 不能以数字开头
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 20;

    pub fn new(username: &str) -> Result<Self> {
        let username = username.trim();

        if username.len() < Self::MIN_LENGTH {
            return Err(InfinityError::Validation {
                field: Some("username".to_string()),
                message: format!("lenght can't less {} 个字符", Self::MIN_LENGTH),
            });
        }
        if username.len() > Self::MAX_LENGTH {
            return Err(InfinityError::Validation {
                field: Some("username".to_string()),
                message: format!("length can't exceed {}", Self::MAX_LENGTH),
            });
        }
        if username.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return Err(InfinityError::Validation {
                field: Some("username".into()),
                message: "不能以数字开头".into(),
            });
        }
        // 格式验证
        if !username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return Err(InfinityError::Validation {
                field: Some("username".to_string()),
                message: format!("只能包含字母、数字和下划线，且不能以数字开头"),
            });
        }

        Ok(Self(username.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Display for Username {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
