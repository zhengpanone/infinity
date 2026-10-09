use axum::Router;

use crate::{router, state};

pub fn app() -> Router {
    router::api_router().with_state(state::AppState::new())
}
