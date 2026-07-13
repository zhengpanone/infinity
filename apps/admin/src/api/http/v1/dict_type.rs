use crate::handlers::dict_type_handler::{
    create, delete as delete_dict_type, detail, exists, page_list, update,
};
use crate::state::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};
pub fn dict_type_routes() -> Router<AppState> {
    Router::new()
        // 管理路由
        .route("/create", post(create))
        .route("/page", post(page_list))
        .route("/detail/{id}", get(detail))
        .route("/update", post(update))
        .route("/delete", delete(delete_dict_type))
        .route("/exists", post(exists))
}
