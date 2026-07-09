use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::types::Json;

use crate::domain::types::ids::UserId;
use crate::enums::user::UserStatus;

/// 用户实体
/// 派生特性说明
/// Debug：用于调试打印
/// Clone：允许创建副本
/// Serialize/Deserialize：JSON 序列化支持
/// FromRow：自动将数据库行转换为 Rust 结构
#[derive(Debug, Clone, FromRow)]
pub struct User {
    // 用户ID
    pub id: UserId,

    /// 用户名
    pub username: String,

    /// 邮箱
    pub email: String,

    /// 手机号
    pub phone: Option<String>,

    /// 用户状态
    pub status: UserStatus,

    /// 密码哈希
    pub password_hash: String,

    /// 显示名称
    pub display_name: String,

    /// 头像URL
    pub avatar_url: Option<String>,

    /// 角色列表
    pub roles: Json<Vec<String>>,

    /// 权限列表
    pub permissions: Json<Vec<String>>,

    /// 是否已验证邮箱
    pub email_verified: bool,

    /// 是否已验证手机
    pub phone_verified: bool,

    /// 用户状态
    // TODO pub status: UserStatus,

    /// 最后登录时间
    pub last_login_at: Option<DateTime<Utc>>,

    /// 登录次数
    pub login_count: i64, // 使用 i64 接收 bigint

    /// 失败登录次数
    pub failed_login_count: i64, // 使用 i64 接收 bigint

    /// 最后失败登录时间
    pub last_failed_login_at: Option<DateTime<Utc>>,

    /// 账户锁定时间
    pub locked_at: Option<DateTime<Utc>>,

    /// 账户锁定到期时间
    pub locked_until: Option<DateTime<Utc>>,

    /// 账户锁定原因
    pub lock_reason: Option<String>,

    /// 密码最后修改时间
    pub password_changed_at: Option<DateTime<Utc>>,

    /// 密码过期时间
    pub password_expires_at: Option<DateTime<Utc>>,

    /// 是否首次登录
    pub is_first_login: bool,

    /// 上次活动时间
    pub last_activity_at: Option<DateTime<Utc>>,

    /// 时区
    pub timezone: Option<String>,

    /// 语言
    pub language: Option<String>,

    /// 元数据
    pub metadata: Option<serde_json::Value>,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 软删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}
