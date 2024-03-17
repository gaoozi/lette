use sqlx::types::chrono::NaiveDateTime;

pub struct Series {
    pub title: String,
    pub desc: Option<String>,
    pub content: String,
    pub cover: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: SeriesStatus,
}

#[derive(Debug, PartialEq)]
pub enum SeriesStatus {
    Created = 0,
    Published = 1,
    Deleted = 2,
}

impl SeriesStatus {
    pub fn create(&mut self) -> anyhow::Result<(), &'static str> {
        match self {
            SeriesStatus::Deleted => return Err("can't active a deleted user"),
            _ => *self = SeriesStatus::Created,
        }

        Ok(())
    }

    pub fn publish(&mut self) -> anyhow::Result<(), &'static str> {
        match self {
            SeriesStatus::Deleted => return Err("can't publish a deleted user"),
            _ => *self = SeriesStatus::Published,
        }

        Ok(())
    }

    pub fn delete(&mut self) {
        *self = SeriesStatus::Deleted
    }
}

impl TryFrom<u8> for SeriesStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SeriesStatus::Created),
            1 => Ok(SeriesStatus::Published),
            2 => Ok(SeriesStatus::Deleted),
            _ => Err(()),
        }
    }
}
