use axum::Router;

pub mod hello_router;

use crate::state::AppState;

pub fn api_router() -> Router<AppState> {
    Router::<AppState>::new().merge(hello_router::router()) // Router<()> 可并入任意 Router<S>
}
