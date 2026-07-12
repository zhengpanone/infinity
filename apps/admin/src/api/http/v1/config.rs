use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{handlers::config_handler::create, state::AppState};

pub fn config_routes() -> Router<AppState> {
    Router::new()
        // 管理路由
        .route("/create", post(create))
    // .route("/page", post(page_list))
    // .route("/detail/{id}", get(detail))
    // .route("/update", post(update))
    // .route("/delete", delete(delete_user))
    // .route("/exists", post(exists))
}
