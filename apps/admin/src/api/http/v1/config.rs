use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::handlers::config_handler::{
    create, delete as delete_config, detail, exists, page_list, update,
};
use crate::state::AppState;

pub fn config_routes() -> Router<AppState> {
    Router::new()
        // 管理路由
        .route("/create", post(create))
        .route("/page", post(page_list))
        .route("/detail/{id}", get(detail))
        .route("/update", post(update))
        .route("/delete", delete(delete_config))
        .route("/exists", post(exists))
}
