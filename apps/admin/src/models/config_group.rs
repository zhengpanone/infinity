use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ConfigGroup {
    /// 分组ID
    pub id: String,

    /// 一级分类编码
    pub category_code: String,

    /// 分组编码
    pub group_code: String,

    /// 分组名称
    pub group_name: String,

    /// 图标
    pub icon: Option<String>,

    /// 排序
    pub order_num: i32,

    /// 备注
    pub remark: Option<String>,

    /// 描述
    pub group_desc: Option<String>,

    /// 是否内置
    pub is_builtin: bool,

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
