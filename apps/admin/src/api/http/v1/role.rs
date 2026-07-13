use crate::{
    handlers::role_handler::{create, delete as delete_role, detail, exists, page_list, update},
    state::AppState,
};
use axum::routing::{delete, get};
use axum::{Router, routing::post};

pub fn role_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create))
        .route("/page", post(page_list))
        .route("/detail/{id}", get(detail))
        .route("/update", post(update))
        .route("/delete", delete(delete_role))
        .route("/exists", post(exists))
}
