macro_rules! define_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, PartialEq, Eq, Hash, sqlx::Type, serde::Serialize, serde::Deserialize,
        )]
        #[sqlx(transparent)]
        #[serde(transparent)]
        pub struct $name(uuid::Uuid);

        impl $name {
            /// 基于新生成的 UUID v7 字符串 ID。
            pub fn generate() -> Self {
                Self(uuid::Uuid::new_v4())
            }
            /// 从已有字符串构造。
            pub fn new(id: uuid::Uuid) -> Self {
                Self(id)
            }
            /// 获取内部 UUID 引用
            pub fn as_uuid(&self) -> uuid::Uuid {
                self.0
            }
            /// 获取所有权
            pub fn into_inner(self) -> uuid::Uuid {
                self.0
            }
        }

        impl From<uuid::Uuid> for $name {
            fn from(s: uuid::Uuid) -> Self {
                Self(s)
            }
        }

        impl From<$name> for uuid::Uuid {
            fn from(value: $name) -> Self {
                value.0
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
define_id!(RoleId);
define_id!(PermissionId);
