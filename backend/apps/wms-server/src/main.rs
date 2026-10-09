use std::net::SocketAddr;

use axum::{Router, routing::get};

async fn hello() -> &'static str {
    "wms-server"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 18081));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
