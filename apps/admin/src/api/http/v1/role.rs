use axum::{Router, routing::post};

use crate::{handlers::role_handler::create, state::AppState};

pub fn role_routers() -> Router<AppState> {
    Router::new().route("/create", post(create))
}
