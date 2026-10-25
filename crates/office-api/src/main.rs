//! Minimal HTTP adapter. Storage wiring is intentionally deferred until the
//! PostgreSQL repository adapter is introduced.

use axum::{routing::get, Json, Router};
use std::net::SocketAddr;

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "service": "office", "status": "ok" }))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(address).await.expect("bind Office API");
    axum::serve(listener, app).await.expect("serve Office API");
}
