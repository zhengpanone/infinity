macro_rules! define_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, Hash, sqlx::Type, serde::Serialize, serde::Deserialize,
        )]
        #[sqlx(transparent)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// 基于新生成的 UUID v7 字符串 ID。
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
            /// 获取所有权
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl From<String> for $name {
            fn from(s: String) -> Self {
                Self(s)
            }
        }

        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                Self(s.to_owned())
            }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
        impl std::borrow::Borrow<str> for $name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

define_id!(UserId);
// define_id!(OrderId);
// define_id!(ProductId);
