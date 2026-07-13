use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    handlers::user_handler::{create, delete as delete_user, detail, exists, page_list, update},
    state::AppState,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        // 管理路由
        .route("/create", post(create))
        .route("/page", post(page_list))
        .route("/detail/{id}", get(detail))
        .route("/update", post(update))
        .route("/delete", delete(delete_user))
        .route("/exists", post(exists))
}
