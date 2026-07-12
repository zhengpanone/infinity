use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "config_type_enum", rename_all = "snake_case")]
pub enum ConfigType {
    String,
    Integer,
    Long,
    Float,
    Double,
    Boolean,
    Json,
    Array,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "config_hint_enum", rename_all = "snake_case")]
pub enum ConfigHint {
    Text,
    Password,
    Number,
    Switch,
    Select,
    Radio,
    Checkbox,
    Textarea,
    Json,
    Date,
    Datetime,
}
