use uuid::Uuid;

#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "TEXT")]
pub enum TaskState {
    Todo,
    Progress,
    Done,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub state: TaskState,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
