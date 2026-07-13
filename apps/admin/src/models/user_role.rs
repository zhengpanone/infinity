use crate::domain::types::ids::SysUserRoleId;
use crate::domain::types::{RoleId, UserId};
use crate::enums::role::UserRoleSource;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct SysUserRole {
    /// 关联ID
    pub id: SysUserRoleId,

    /// 用户ID
    pub user_id: UserId,

    /// 角色ID
    pub role_id: RoleId,

    /// 来源：1-手动分配，2-自动分配，3-继承
    pub source: UserRoleSource,

    /// 生效时间
    pub effective_from: Option<DateTime<Utc>>,

    /// 失效时间
    pub effective_to: Option<DateTime<Utc>>,

    /// 创建人ID
    pub created_id: String,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 创建人
    pub created_by: String,

    /// 最后修改人ID
    pub updated_id: String,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 最后修改人
    pub updated_by: String,

    /// 是否已删除
    pub is_deleted: bool,

    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}
