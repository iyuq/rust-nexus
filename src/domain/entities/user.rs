use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain entity representing a User
/// This is the core business entity, free from infrastructure concerns
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    id: UserId,
    name: UserName,
    email: Email,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Value object for User ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

/// Value object for User Name with validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

/// Value object for Email with validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl User {
    /// Create a new User (factory method)
    pub fn new(name: UserName, email: Email) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            name,
            email,
            created_at: now,
            updated_at: now,
        }
    }

    /// Reconstruct User from persistence (used by adapters)
    pub fn from_persistence(
        id: UserId,
        name: UserName,
        email: Email,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            email,
            created_at,
            updated_at,
        }
    }

    /// Update user information
    pub fn update(&mut self, name: Option<UserName>, email: Option<Email>) -> Result<(), UserError> {
        if let Some(new_name) = name {
            self.name = new_name;
        }
        if let Some(new_email) = email {
            self.email = new_email;
        }
        self.updated_at = Utc::now();
        Ok(())
    }

    // Getters
    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl UserName {
    pub fn new(name: String) -> Result<Self, UserError> {
        if name.trim().is_empty() {
            return Err(UserError::InvalidName("Name cannot be empty".to_string()));
        }
        if name.len() > 100 {
            return Err(UserError::InvalidName("Name cannot exceed 100 characters".to_string()));
        }
        Ok(Self(name.trim().to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn new(email: String) -> Result<Self, UserError> {
        let email = email.trim().to_lowercase();
        if email.is_empty() {
            return Err(UserError::InvalidEmail("Email cannot be empty".to_string()));
        }
        if !email.contains('@') || !email.contains('.') {
            return Err(UserError::InvalidEmail("Invalid email format".to_string()));
        }
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Domain errors for User operations
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
    #[error("User not found")]
    NotFound,
    #[error("Email already exists")]
    EmailAlreadyExists,
}
