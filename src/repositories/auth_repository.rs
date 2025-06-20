use anyhow::Result;
use sqlx::{Pool, Postgres, query};

pub struct AuthRepository {
    pool: Pool<Postgres>,
}

impl AuthRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, email: &str, password: &str) -> Result<i64, sqlx::Error> {
        let record = query!(
            r#"INSERT INTO users (email, password) VALUES($1, $2) RETURNING id"#,
            email,
            password
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(record.id)
    }

    pub async fn get_auth_user_by_email(&self, email: &str) -> Result<(i64, String), sqlx::Error> {
        let record = query!(r#"SELECT id, password FROM users WHERE email = $1"#, email)
            .fetch_one(&self.pool)
            .await?;

        Ok((record.id, record.password))
    }
}
