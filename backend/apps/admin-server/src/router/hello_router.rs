use axum::{Router, routing::get};

use crate::{
    handler::hello_handler::{get_todo, hello, index},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/hello", get(hello))
        .route("/todo/{id}", get(get_todo))
}
