use crate::user_gen::{CreateUserRequest, UserResponse};
use crate::validate::{Validate, ValidationError};
use sqlx::prelude::FromRow;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, sqlx::Type, serde::Deserialize, serde::Serialize)]
#[sqlx(transparent)]
pub struct Id(i32);

#[derive(Debug, FromRow)]
pub struct User {
    pub id: Id,
    pub username: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id.into(),
            username: value.username,
            email: value.email,
        }
    }
}

impl From<i32> for Id {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Id> for i32 {
    fn from(value: Id) -> Self {
        value.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Validate for CreateUserRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.email.is_empty() {
            return Err(ValidationError::MissingEmail);
        }

        if self.username.is_empty() {
            return Err(ValidationError::MissingUsername);
        }

        Ok(())
    }
}
