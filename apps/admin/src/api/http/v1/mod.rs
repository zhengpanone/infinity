use axum::Router;

use crate::state::AppState;

pub mod config;
pub mod config_category;
pub mod config_group;
pub mod dict_item;
pub mod dict_type;
pub mod role;
pub mod user;

pub fn v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/user", user::user_routes())
        .nest("/role", role::role_routes())
        .nest(
            "/config_category",
            config_category::config_category_routes(),
        )
        .nest("/config_group", config_group::config_group_routes())
        .nest("/config", config::config_routes())
        .nest("/dict_type", dict_type::dict_type_routes())
        .nest("/dict_item", dict_item::dict_item_routes())
}
