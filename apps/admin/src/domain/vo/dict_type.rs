use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::domain::vo::dict_item::DictItemVO;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DictTypeVO {
    pub code: String,

    pub name: String,

    pub items: Vec<DictItemVO>,
}