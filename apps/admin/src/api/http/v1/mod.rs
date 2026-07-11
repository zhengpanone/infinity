use axum::Router;

use crate::state::AppState;

pub mod role;
pub mod user;

pub fn v1_routes() -> Router<AppState> {
    Router::new().nest("/user", user::user_admin_routes())
}
