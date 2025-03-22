use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

fn router(pool: Arc<sqlx::PgPool>) -> Router {
    Router::new()
        .route("/tasks", get(|| async { "Hello World!" }))
        .route("/tasks/{id}", get(|| async { "H" }))
        .route("/tasks", post(handle_create))
        .layer(Extension(pool))
        .route("/tasks/{id}", patch(|| async { "Hello World!" }))
        .route("/tasks/{id}", delete(|| async { "Hello World!" }))
        .fallback(|| async { "Not Found" })
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum TaskState {
    Todo,
    Progress,
    Done,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub state: TaskState,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
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

#[derive(Debug, Clone, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub state: TaskState,
}

#[axum::debug_handler]
async fn handle_create(
    Extension(pool): Extension<Arc<sqlx::PgPool>>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let id = Uuid::new_v4();
    let task = sqlx::query_as!(
        Task,
        r#"
            insert into
                task (id, title, state)
            values
                ($1, $2, $3)
            returning
                id,
                title,
                state as "state: _",
                created_at,
                updated_at
        "#,
        id,
        payload.title,
        payload.state as _,
    )
    .fetch_one(&*pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(task))
}
