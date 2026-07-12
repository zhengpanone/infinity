use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct SysDictItemI18n {
    /// ID
    pub id: Uuid,

    /// 字典项ID
    pub item_id: Uuid,

    /// 语言
    pub locale: String,

    /// 名称
    pub label: String,

    /// 描述
    pub description: Option<String>,
}