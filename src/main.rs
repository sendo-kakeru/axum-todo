use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Home" }))
        .route("/hello", get(|| async { "Hello World!" }))
}

#[tokio::main]
async fn main() {
    let app = router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
