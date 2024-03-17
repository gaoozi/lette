use sqlx::types::chrono::NaiveDateTime;

use super::Status;

pub struct Friend {
    pub title: String,
    pub url: String,
    pub remark: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: Status,
}
