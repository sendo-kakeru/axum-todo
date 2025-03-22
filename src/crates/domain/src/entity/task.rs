use uuid::Uuid;
use chrono::{DateTime, Utc, FixedOffset, TimeZone};

pub enum TaskState {
    Todo,
    Progress,
    Done,
}

pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub state: TaskState,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
