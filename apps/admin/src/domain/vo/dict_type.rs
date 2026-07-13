use crate::domain::vo::dict_item::SysDictItemVO;
use crate::models::dict_type::SysDictType;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DictTypeVO {
    pub code: String,

    pub name: String,

    pub items: Vec<SysDictItemVO>,
}

impl From<SysDictType> for DictTypeVO {
    fn from(dict_type: SysDictType) -> Self {
        Self {
            code: "".to_string(),
            name: "".to_string(),
            items: vec![],
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DictTypeExistsVO {
    pub category_code_exists: bool,
}
