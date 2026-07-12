use axum::Router;

use crate::state::AppState;

pub mod config;
pub mod role;
pub mod user;
mod dict;

pub fn v1_routes() -> Router<AppState> {
    Router::new().nest("/user", user::user_admin_routes())
}
