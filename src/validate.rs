use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum ValidationError {
    MissingEmail,
    MissingUsername,
}

impl ValidationError {
    fn message(&self) -> &'static str {
        match self {
            ValidationError::MissingEmail => "email is required",
            ValidationError::MissingUsername => "username is required",
        }
    }
}

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.message()).into_response()
    }
}

impl From<ValidationError> for StatusCode {
    fn from(_: ValidationError) -> Self {
        StatusCode::BAD_REQUEST
    }
}
