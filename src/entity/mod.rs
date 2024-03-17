pub mod article;
pub mod category;
pub mod friend;
pub mod notice;
pub mod series;
pub mod tag;
pub mod user;

#[derive(Debug, PartialEq)]
pub enum Status {
    Normal = 0,
    Offline = 1,
}

impl Status {
    pub fn normal(&mut self) {
        *self = Status::Normal
    }

    pub fn offline(&mut self) {
        *self = Status::Offline
    }
}

impl TryFrom<u8> for Status {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Normal),
            1 => Ok(Status::Offline),
            _ => Err(()),
        }
    }
}
