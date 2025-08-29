use async_trait::async_trait;

use crate::domain::entities::{User, UserId, Email, UserError};

/// Port (interface) for User repository operations
/// This defines the contract that infrastructure adapters must implement
#[async_trait]
pub trait UserRepositoryPort: Send + Sync + Clone {
    /// Save a new user
    async fn save(&self, user: &User) -> Result<(), UserError>;
    
    /// Find user by ID
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError>;
    
    /// Update an existing user
    async fn update(&self, user: &User) -> Result<(), UserError>;
    
    /// Delete a user by ID
    async fn delete(&self, id: &UserId) -> Result<(), UserError>;
    
    /// Get all users with pagination
    async fn find_all(&self, offset: i64, limit: i64) -> Result<Vec<User>, UserError>;
    
    /// Check if user exists by email
    async fn exists_by_email(&self, email: &Email) -> Result<bool, UserError>;
}
