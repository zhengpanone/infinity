use crate::models::config_group::ConfigGroup;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct ConfigGroupVO {
    pub id: Uuid,

    pub group_code: String,

    pub group_name: String,

    /// 分组图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// 排序
    pub order_num: i32,

    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,

    /// 分类描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_desc: Option<String>,

    /// 是否系统内置
    pub is_builtin: bool,

    pub created_id: String,
    pub create_by: String,
    pub created_at: DateTime<Utc>,

    pub updated_id: String,
    pub updated_by: String,
    pub updated_at: DateTime<Utc>,
}

impl From<ConfigGroup> for ConfigGroupVO {
    fn from(value: ConfigGroup) -> Self {
        Self {
            id: value.id.as_uuid(),
            group_code: value.group_code,
            group_name: value.group_name,
            icon: value.icon,
            order_num: value.order_num,
            remark: value.remark,
            group_desc: value.group_desc,
            is_builtin: value.is_builtin,
            created_id: value.created_id,
            create_by: value.created_by,
            created_at: value.created_at,
            updated_id: value.updated_id,
            updated_by: value.updated_by,
            updated_at: value.updated_at,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConfigGroupExistsVO {
    pub username_exists: Option<bool>,

    pub email_exists: Option<bool>,

    pub phone_exists: Option<bool>,
}
