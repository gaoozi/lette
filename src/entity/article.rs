use secrecy::Secret;
use sqlx::types::chrono::NaiveDateTime;

pub struct Article {
    pub title: String,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub content: String,
    pub html: String,
    pub cover: Option<String>,
    pub password: Option<Secret<String>>,
    pub is_top: i8,
    pub read_count: i32,
    pub like_count: i32,
    pub category_id: i32,
    pub author_id: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: ArticleStatus,
}

#[derive(Debug, PartialEq)]
pub enum ArticleStatus {
    Created = 0,
    Published = 1,
    Deleted = 2,
}

impl ArticleStatus {
    pub fn create(&mut self) -> anyhow::Result<(), &'static str> {
        match self {
            ArticleStatus::Deleted => return Err("can't active a deleted user"),
            _ => *self = ArticleStatus::Created,
        }

        Ok(())
    }

    pub fn publish(&mut self) -> anyhow::Result<(), &'static str> {
        match self {
            ArticleStatus::Deleted => return Err("can't publish a deleted user"),
            _ => *self = ArticleStatus::Published,
        }

        Ok(())
    }

    pub fn delete(&mut self) {
        *self = ArticleStatus::Deleted
    }
}

impl TryFrom<u8> for ArticleStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ArticleStatus::Created),
            1 => Ok(ArticleStatus::Published),
            2 => Ok(ArticleStatus::Deleted),
            _ => Err(()),
        }
    }
}
