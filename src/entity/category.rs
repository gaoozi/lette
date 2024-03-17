use sqlx::types::chrono::NaiveDateTime;

use super::Status;

pub struct Category {
    pub name: String,
    pub icon: Option<String>,
    pub desc: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: Status,
}
