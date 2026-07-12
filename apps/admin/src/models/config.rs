use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::prelude::FromRow;

use crate::enums::config::{ConfigHint, ConfigType};

#[derive(Debug, Clone, FromRow)]
pub struct Config {
    /// 配置ID
    pub id: String,

    /// 一级分类编码
    pub category_code: String,

    /// 二级分组编码
    pub group_code: String,

    /// 配置Key
    pub config_key: String,

    /// 配置名称
    pub config_name: String,

    /// 配置值
    pub config_value: Option<String>,

    /// 默认值
    pub default_value: Option<String>,

    /// 配置类型
    pub config_type: ConfigType,

    /// 前端展示类型
    pub value_hint: ConfigHint,

    /// 单位
    pub value_unit: Option<String>,

    /// 校验规则
    pub validation_rule: Option<String>,

    /// 下拉选项
    pub options: Value,

    /// 是否显示
    pub is_visible: bool,

    /// 是否可编辑
    pub is_editable: bool,

    /// 是否系统内置
    pub is_builtin: bool,

    /// 是否加密
    pub is_encrypted: bool,

    /// 乐观锁版本
    pub version: i64,

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
