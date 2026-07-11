use core::fmt;
use std::str::FromStr;

use infinity_error::{InfinityError, Result};
use serde::{Deserialize, Serialize};
use validator::ValidateUrl;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct Url(String);

impl Url {
    const MAX_LENGTH: usize = 500;
    pub fn new(url: impl AsRef<str>) -> Result<Self> {
        let url = url.as_ref().trim();
        if url.is_empty() {
            return Err(InfinityError::validation("url cannot none"));
        }
        if url.chars().count() > Self::MAX_LENGTH {
            return Err(InfinityError::validation(format!(
                "url length cannot large {} chartset",
                Self::MAX_LENGTH
            )));
        }
        if !url.validate_url() {
            return Err(InfinityError::validation(format!(
                "invalidation url: {}",
                url
            )));
        }
        let lowercase = url.to_ascii_lowercase();

        if !lowercase.starts_with("http://") && !lowercase.starts_with("https://") {
            return Err(InfinityError::validation_field(
                "avatar_url",
                "头像地址只支持 HTTP 或 HTTPS",
            ));
        }
        Ok(Self(url.to_string()))
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

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Url {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl From<Url> for String {
    fn from(url: Url) -> Self {
        url.0
    }
}

impl FromStr for Url {
    type Err = InfinityError;

    fn from_str(value: &str) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Url {
    type Error = InfinityError;
    fn try_from(value: &str) -> core::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for Url {
    type Error = InfinityError;
    fn try_from(value: String) -> core::result::Result<Self, Self::Error> {
        Self::new(&value)
    }
}
