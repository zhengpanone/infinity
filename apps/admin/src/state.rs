use std::sync::Arc;

use infinity_database::Database;

use crate::services::Services;

/// 处理器共享状态。
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub services: Arc<Services>,
}
