use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::domain::{
    User, UserId, UserName, Email, UserError,
    ports::UserRepositoryPort,
};

/// Database adapter implementing UserRepositoryPort
#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

/// Database model for User (infrastructure concern)
#[derive(Debug, FromRow)]
struct UserDbModel {
    id: Uuid,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepositoryPort for PostgresUserRepository {
    async fn save(&self, user: &User) -> Result<(), UserError> {
        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.name().as_str())
        .bind(user.email().as_str())
        .bind(user.created_at())
        .bind(user.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save user: {}", e);
            UserError::InvalidEmail("Database error".to_string())
        })?;

        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError> {
        let result = sqlx::query_as::<_, UserDbModel>(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find user by id: {}", e);
            UserError::InvalidEmail("Database error".to_string())
        })?;

        match result {
            Some(db_user) => Ok(Some(db_user.to_domain()?)),
            None => Ok(None),
        }
    }

    async fn update(&self, user: &User) -> Result<(), UserError> {
        let result = sqlx::query(
            r#"
            UPDATE users 
            SET name = $2, email = $3, updated_at = $4
            WHERE id = $1
            "#,
        )
        .bind(user.id().as_uuid())
        .bind(user.name().as_str())
        .bind(user.email().as_str())
        .bind(user.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update user: {}", e);
            UserError::InvalidEmail("Database error".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(UserError::NotFound);
        }

        Ok(())
    }

    async fn delete(&self, id: &UserId) -> Result<(), UserError> {
        let result = sqlx::query(
            "DELETE FROM users WHERE id = $1",
        )
        .bind(id.as_uuid())
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete user: {}", e);
            UserError::InvalidEmail("Database error".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(UserError::NotFound);
        }

        Ok(())
    }

    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<User>, UserError> {
        let results = sqlx::query_as::<_, UserDbModel>(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find all users: {}", e);
            UserError::InvalidEmail("Database error".to_string())
        })?;

        results.into_iter()
            .map(|db_user| db_user.to_domain())
            .collect()
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, UserError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
        )
        .bind(email.as_str())
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check email existence: {}", e);
            UserError::InvalidEmail("Database error".to_string())
        })?;

        Ok(result.0)
    }
}

impl UserDbModel {
    fn to_domain(self) -> Result<User, UserError> {
        let id = UserId::from_uuid(self.id);
        let name = UserName::new(self.name)?;
        let email = Email::new(self.email)?;
        
        Ok(User::from_persistence(
            id,
            name,
            email,
            self.created_at,
            self.updated_at,
        ))
    }
}
