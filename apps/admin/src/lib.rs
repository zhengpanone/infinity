pub mod api;
pub mod domain;
pub mod enums;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod server;
pub mod services;
pub mod startup;
pub mod state;
pub mod telemetry;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
