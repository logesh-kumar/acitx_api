use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::{types::Uuid, PgPool, Result};

use crate::models::User;

impl User {
    pub async fn create(pool: &PgPool, username: &str, password: &str) -> Result<Self> {
        let password_hash = match hash(password, DEFAULT_COST) {
            Ok(hash) => hash,
            Err(_) => {
                return Err(sqlx::Error::Decode(
                    "Failed to hash the password".to_string().into(),
                ))
            }
        };

        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id, username, password_hash) VALUES ($1, $2, $3) RETURNING *",
            Uuid::new_v4(),
            username,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<Option<Self>> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(pool)
            .await?;

        Ok(user)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }
}
