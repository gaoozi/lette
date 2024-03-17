use super::Status;
use sqlx::types::chrono::NaiveDateTime;

pub struct Notice {
    pub title: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: Status,
}
