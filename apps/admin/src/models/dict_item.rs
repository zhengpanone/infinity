
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// 字典项
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysDictItem {
    /// 字典项ID
    pub id: Uuid,

    /// 字典类型ID
    pub dict_id: Uuid,

    /// 字典类型编码
    pub dict_code: String,

    /// 字典项编码
    pub item_code: String,

    /// 字典项名称
    pub item_label: String,

    /// 字典值
    pub item_value: String,

    /// 标签颜色
    pub item_color: Option<String>,

    /// 图标
    pub icon: Option<String>,

    /// CSS样式
    pub css_class: Option<String>,

    /// 扩展属性
    pub ext_data: Value,

    /// 是否默认
    pub is_default: bool,

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