use sqlx::{PgPool, Error};
use crate::models::*;

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        UserRepository { pool }
    }

    pub async fn create(&self, email: &str, password: &str) -> Result<User, Error> {
        User::create(self.pool, email, password).await
    }

    pub async fn soft_delete(&self, id: i32) -> Result<(), Error> {
        User::soft_delete(self.pool, id).await
    }

    pub async fn find_all(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query_as!(User, "SELECT * FROM users WHERE is_deleted = false")
            .fetch_all(self.pool)
            .await?;
        Ok(users)
    }
}