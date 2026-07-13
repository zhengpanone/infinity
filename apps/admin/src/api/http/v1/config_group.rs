use crate::handlers::config_category_handler::create;
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};
use crate::handlers::config_group_handler::{detail, exists, page_list, update,delete as delete_config_group};

pub fn config_group_routes() -> Router<AppState> {
    Router::new()
        // 管理路由
        .route("/create", post(create))
    .route("/page", post(page_list))
    .route("/detail/{id}", get(detail))
    .route("/update", post(update))
    .route("/delete", delete(delete_config_group))
    .route("/exists", post(exists))
}
