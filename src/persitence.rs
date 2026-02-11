use crate::{
    user::{Id, User},
    user_gen::CreateUserRequest,
};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub enum PersistenceError {
    UniqueViolation,
    DatabaseError(Box<dyn std::error::Error + Send + Sync>),
}

impl From<sqlx::Error> for PersistenceError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(err) if err.is_unique_violation() => {
                PersistenceError::UniqueViolation
            }
            _ => PersistenceError::DatabaseError(Box::new(error)),
        }
    }
}

#[derive(Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn connect(url: &str, pool_size: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn find_user_by_id(&self, id: Id) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as(
            "
            SELECT id, username, email
            FROM users
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn save(&self, user: CreateUserRequest) -> Result<Id, PersistenceError> {
        sqlx::query!(
            "
            INSERT INTO users (username, email)
            VALUES ($1, $2)
            RETURNING id
            ",
            user.username.as_str(),
            user.email.as_str(),
        )
        .fetch_one(&self.pool)
        .await
        .map(|row| row.id.into())
        .map_err(PersistenceError::from)
    }

    pub async fn delete_by_id(&self, id: Id) -> Result<bool, PersistenceError> {
        let result = sqlx::query!("DELETE FROM users WHERE id = $1", id as Id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
