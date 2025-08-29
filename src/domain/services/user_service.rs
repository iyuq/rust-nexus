use crate::domain::{
    entities::{User, UserName, Email, UserError},
    ports::UserRepositoryPort,
};

/// Domain service for User business logic
/// Contains business rules that don't naturally fit in entities
#[derive(Clone)]
pub struct UserDomainService<R: UserRepositoryPort> {
    user_repository: R,
}

impl<R: UserRepositoryPort> UserDomainService<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    /// Create a new user with business validation
    pub async fn create_user(&self, name: UserName, email: Email) -> Result<User, UserError> {
        // Business rule: Check if email already exists
        if self.user_repository.exists_by_email(&email).await? {
            return Err(UserError::EmailAlreadyExists);
        }

        // Create the user entity
        let user = User::new(name, email);
        
        // Save the user
        self.user_repository.save(&user).await?;
        
        Ok(user)
    }

    /// Update user with business validation
    pub async fn update_user(
        &self,
        user: &mut User,
        new_name: Option<UserName>,
        new_email: Option<Email>,
    ) -> Result<(), UserError> {
        // Business rule: If email is being changed, check uniqueness
        if let Some(ref email) = new_email {
            if email != user.email() {
                if self.user_repository.exists_by_email(email).await? {
                    return Err(UserError::EmailAlreadyExists);
                }
            }
        }

        // Update the entity
        user.update(new_name, new_email)?;
        
        // Persist changes
        self.user_repository.update(user).await?;
        
        Ok(())
    }
}
