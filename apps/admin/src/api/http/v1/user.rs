use axum::{Router, routing::get};

use crate::{handlers::user_handler::create_user, state::AppState};

pub fn user_admin_routes() -> Router<AppState> {
    Router::new()
        // 管理路由
        .route("/", get(create_user))
}
