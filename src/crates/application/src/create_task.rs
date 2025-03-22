use axum::http;
use domain::entity::task::{Task, TaskState};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub state: TaskState,
}

#[axum::debug_handler]
pub async fn handle_create(
    axum::Extension(pool): axum::Extension<Arc<sqlx::PgPool>>,
    axum::Json(payload): axum::Json<CreateTask>,
) -> Result<axum::Json<Task>, (http::StatusCode, String)> {
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
    .map_err(|e| (http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(axum::Json(task))
}
