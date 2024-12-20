// src/structures/validation.rs

use std::fmt;

/// Custom error type for validation failures.
#[derive(Debug)]
pub struct ValidationError {
    field: String,
    message: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str) -> Self {
        ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation failed for field '{}': {}", self.field, self.message)
    }
}

impl fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ValidationError {{ field: '{}', message: '{}' }}", self.field, self.message)
    }
}

pub type ValidationResult = Result<(), ValidationError>;

pub trait Validatable {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}
