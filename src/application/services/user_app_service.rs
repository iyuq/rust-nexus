use uuid::Uuid;

use crate::{
    application::dto::{CreateUserDto, UpdateUserDto, UserResponseDto},
    domain::{UserDomainService, UserRepositoryPort, UserId, UserError},
};

/// Application service for User use cases
/// Orchestrates domain services and handles cross-cutting concerns
#[derive(Clone)]
pub struct UserApplicationService<R: UserRepositoryPort> {
    domain_service: UserDomainService<R>,
    repository: R,
}

impl<R: UserRepositoryPort> UserApplicationService<R> {
    pub fn new(repository: R) -> Self {
        let domain_service = UserDomainService::new(repository.clone());
        Self {
            domain_service,
            repository,
        }
    }

    /// Create a new user
    pub async fn create_user(&self, dto: CreateUserDto) -> Result<UserResponseDto, UserError> {
        let (name, email) = dto.to_domain()?;
        let user = self.domain_service.create_user(name, email).await?;
        Ok(UserResponseDto::from(&user))
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<UserResponseDto>, UserError> {
        let user_id = UserId::from_uuid(id);
        if let Some(user) = self.repository.find_by_id(&user_id).await? {
            Ok(Some(UserResponseDto::from(&user)))
        } else {
            Ok(None)
        }
    }

    /// Update user
    pub async fn update_user(&self, id: Uuid, dto: UpdateUserDto) -> Result<UserResponseDto, UserError> {
        let user_id = UserId::from_uuid(id);
        let mut user = self.repository.find_by_id(&user_id).await?
            .ok_or(UserError::NotFound)?;
        
        let (name, email) = dto.to_domain()?;
        self.domain_service.update_user(&mut user, name, email).await?;
        
        Ok(UserResponseDto::from(&user))
    }

    /// Delete user
    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        let user_id = UserId::from_uuid(id);
        
        // Check if user exists
        if self.repository.find_by_id(&user_id).await?.is_none() {
            return Err(UserError::NotFound);
        }
        
        self.repository.delete(&user_id).await
    }

    /// Get all users with pagination
    pub async fn get_all_users(&self, page: Option<i64>, limit: Option<i64>) -> Result<Vec<UserResponseDto>, UserError> {
        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(10);
        let offset = page * limit;
        
        let users = self.repository.find_all(offset, limit).await?;
        Ok(users.iter().map(UserResponseDto::from).collect())
    }
}
