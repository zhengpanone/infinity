use core::fmt;
use std::str::FromStr;

use infinity_error::{InfinityError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Phone(String);

impl Phone {
    const MAX_LENGTH: usize = 500;
    pub fn new(phone: impl AsRef<str>) -> Result<Self> {
        let phone = phone.as_ref().trim();
        if phone.is_empty() {
            return Err(InfinityError::validation("phone number cannot none"));
        }
        if phone.chars().count() > Self::MAX_LENGTH {
            return Err(InfinityError::validation(format!(
                "phone number length cannot large {} chartset",
                Self::MAX_LENGTH
            )));
        }

        Ok(Self(phone.to_string()))
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

impl AsRef<str> for Phone {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Phone {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<Phone> for String {
    fn from(phone: Phone) -> Self {
        phone.0
    }
}

impl FromStr for Phone {
    type Err = InfinityError;

    fn from_str(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Phone {
    type Error = InfinityError;
    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Phone {
    type Error = InfinityError;
    fn try_from(value: String) -> core::result::Result<Self, Self::Error> {
        Self::new(&value)
    }
}
