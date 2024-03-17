use sqlx::MySqlPool;

use crate::entity::user::CreateUserData;

pub struct UserRepository {
    pool: MySqlPool,
}

impl UserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    async fn create(&self, user_data: CreateUserData) {}
}
