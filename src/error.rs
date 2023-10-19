use crate::models::password_requirements::PasswordRequirements;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    AuthError(AuthError),
    DatabaseError(String),
    DatabaseConnectionError(String),
    UserNameTaken(String),
    UserNotFound(String),
    InvalidCredentials,
    UserNotConfirmed(String),
    ValidationError(ValidationError),
    Status(u16, String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[cfg(feature = "r2d2")]
impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self::DatabaseConnectionError(e.to_string())
    }
}

#[cfg(feature = "diesel")]
impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self::DatabaseError(e.to_string())
    }
}

impl From<AuthError> for Error {
    fn from(value: AuthError) -> Self {
        Error::AuthError(value)
    }
}

#[cfg(feature = "rocket")]
impl From<rocket::http::Status> for Error {
    fn from(value: rocket::http::Status) -> Self {
        Error::Status(value.code, value.to_string())
    }
}

#[cfg(feature = "azure_core")]
impl From<azure_core::error::Error> for Error {
    fn from(value: azure_core::error::Error) -> Self {
        Error::Status(500, value.to_string())
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthError {
    MissingClaim(String),
    InvalidFormat(String),
    TokenExpiredS(i64),
    JwtError(String),
}

#[cfg(feature = "jwt")]
impl From<jwt::Error> for AuthError {
    fn from(value: jwt::Error) -> Self {
        Self::JwtError(value.to_string())
    }
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ValidationError {
    Username(UsernameValidationError),
    Password(PasswordRequirements),
    Query(QueryValidationError),
    ResourceData(ResourceDataValidationError),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UsernameValidationError {
    InvalidLength(i32, i32),
    InvalidCharacters(Vec<char>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum QueryValidationError {
    InvalidColumn(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResourceDataValidationError {
    KeyMismatch(String, String),
    KeyMissing,
    ValueMissing,
}
