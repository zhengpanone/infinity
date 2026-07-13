use crate::models::dict_item::SysDictItem;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SysDictItemVO {
    pub id: Uuid,

    pub dict_code: String,

    pub item_code: String,

    pub label: String,

    pub value: String,

    pub icon: Option<String>,

    pub is_default: bool,

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

impl From<SysDictItem> for SysDictItemVO {
    fn from(dict_item: SysDictItem) -> Self {
        Self {
            id: dict_item.id,
            dict_code: dict_item.dict_code,
            item_code: dict_item.item_code,
            label: "".to_string(),
            icon: dict_item.icon,

            created_id: dict_item.created_id,
            created_by: dict_item.created_by,
            created_at: dict_item.created_at,
            updated_id: dict_item.updated_id,
            updated_by: dict_item.updated_by,
            is_deleted: false,
            updated_at: dict_item.updated_at,
            value: "".to_string(),
            is_default: false,
            deleted_at: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SysDictItemExistsVO {
    pub category_code_exists: bool,
}
