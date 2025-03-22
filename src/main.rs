use std::sync::Arc;

use application::create_task::handle_create;
use axum::routing;

fn router(pool: Arc<sqlx::PgPool>) -> axum::Router {
    axum::Router::new()
        .route("/tasks", routing::get(|| async { "Hello World!" }))
        .route("/tasks/{id}", routing::get(|| async { "H" }))
        .route("/tasks", routing::post(handle_create))
        .layer(axum::Extension(pool))
        .route("/tasks/{id}", routing::patch(|| async { "Hello World!" }))
        .route("/tasks/{id}", routing::delete(|| async { "Hello World!" }))
        .fallback(|| async { "Not Found" })
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Arc::new(sqlx::PgPool::connect(&database_url).await.unwrap());

    let app = router(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
