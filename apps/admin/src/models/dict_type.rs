use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// 字典类型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysDictType {
    /// 字典类型ID
    pub id: Uuid,

    /// 字典编码
    pub dict_code: String,

    /// 字典名称
    pub dict_name: String,

    /// 图标
    pub icon: Option<String>,

    /// 主题颜色
    pub color: Option<String>,

    /// 描述
    pub description: Option<String>,

    /// 是否系统内置
    pub is_builtin: bool,

    /// 是否启用
    pub is_enabled: bool,

    /// 排序
    pub order_num: i32,

    /// 备注
    pub remark: Option<String>,

    /// 创建人ID
    pub created_id: String,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 创建人
    pub created_by: String,

    /// 更新人ID
    pub updated_id: String,

    /// 更新时间
    pub updated_at: DateTime<Utc>,

    /// 更新人
    pub updated_by: String,

    /// 是否删除
    pub is_deleted: bool,

    /// 删除时间
    pub deleted_at: Option<DateTime<Utc>>,
}


