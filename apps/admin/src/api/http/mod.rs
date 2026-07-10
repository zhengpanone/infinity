use axum::{
    Router,
    routing::post,
};

use crate::{handlers::user_handler::create_user, state::AppState};

pub mod v1;

pub fn v1_routes() -> Router<AppState> {
    Router::new().route("/admin/user", post(create_user))
}
