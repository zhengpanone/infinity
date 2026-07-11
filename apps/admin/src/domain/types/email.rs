use core::fmt;
use std::str::FromStr;

use infinity_error::{InfinityError, Result};
use serde::{Deserialize, Serialize};
use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Email(String);

impl Email {
    const MAX_LENGTH: usize = 100;
    pub fn new(email: impl AsRef<str>) -> Result<Self> {
        let email = email.as_ref().trim().to_ascii_lowercase();
        if email.is_empty() {
            return Err(InfinityError::validation("email cannot none"));
        }
        if email.chars().count() > Self::MAX_LENGTH {
            return Err(InfinityError::validation(format!(
                "email length cannot large {} chartset",
                Self::MAX_LENGTH
            )));
        }
        if !email.validate_email() {
            return Err(InfinityError::validation(format!(
                "invalidation email: {}",
                email
            )));
        }
        Ok(Self(email))
    }
    /// 获取邮箱字符串引用
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    /// 获取邮箱域名
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
    pub fn local_part(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Email {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self.0)
        formatter.write_str(self.as_str())
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

impl FromStr for Email {
    type Err = InfinityError;

    fn from_str(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Email {
    type Error = InfinityError;
    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Email {
    type Error = InfinityError;
    fn try_from(value: String) -> core::result::Result<Self, Self::Error> {
        Self::new(&value)
    }
}
