use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{User, UserName, Email, UserError};

/// DTO for creating a user
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
    pub email: String,
}

/// DTO for updating a user
#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub name: Option<String>,
    pub email: Option<String>,
}

/// DTO for user response
#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DTO for API responses
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl CreateUserDto {
    /// Convert DTO to domain value objects
    pub fn to_domain(self) -> Result<(UserName, Email), UserError> {
        let name = UserName::new(self.name)?;
        let email = Email::new(self.email)?;
        Ok((name, email))
    }
}

impl UpdateUserDto {
    /// Convert DTO to domain value objects
    pub fn to_domain(self) -> Result<(Option<UserName>, Option<Email>), UserError> {
        let name = if let Some(n) = self.name {
            Some(UserName::new(n)?)
        } else {
            None
        };
        
        let email = if let Some(e) = self.email {
            Some(Email::new(e)?)
        } else {
            None
        };
        
        Ok((name, email))
    }
}

impl From<&User> for UserResponseDto {
    fn from(user: &User) -> Self {
        Self {
            id: user.id().as_uuid(),
            name: user.name().as_str().to_string(),
            email: user.email().as_str().to_string(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
