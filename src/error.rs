use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg(feature = "api-errors")]
pub type ApiError<'a> = rocket::response::status::Custom<
    rocket::serde::json::Json<crate::models::api_response::ApiResponse<'a, Error>>,
>;

#[cfg(feature = "api-errors")]
impl From<Error> for ApiError<'_> {
    fn from(value: Error) -> Self {
        rocket::response::status::Custom(
            value.status(),
            rocket::serde::json::Json(crate::models::api_response::ApiResponse::err(value)),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Error {
    Auth(AuthError),
    Database(String),
    DatabaseConnection(String),
    User(UserError),
    Register(RegisterError),
    Validation(ValidationError),
    Status(u16, String),
}

#[cfg(feature = "api-errors")]
impl Error {
    fn status(&self) -> rocket::http::Status {
        match self {
            Error::Auth(e) => match e {
                AuthError::JwtError(_) => rocket::http::Status::InternalServerError,
                _ => rocket::http::Status::BadRequest,
            },
            Error::Database(_) => rocket::http::Status::InternalServerError,
            Error::DatabaseConnection(_) => rocket::http::Status::InternalServerError,
            Error::User(_) | Error::Register(_) => rocket::http::Status::Unauthorized,
            Error::Validation(_) => rocket::http::Status::BadRequest,
            Error::Status(status, _) => rocket::http::Status::from_code(*status).unwrap(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[cfg(feature = "r2d2")]
impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self::DatabaseConnection(e.to_string())
    }
}

#[cfg(feature = "rocket")]
#[cfg(feature = "diesel")]
impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::DatabaseError(k, e) => match k {
                diesel::result::DatabaseErrorKind::Unknown => {
                    Self::Database(e.message().to_string())
                }
                _ => Self::Status(
                    rocket::http::Status::BadRequest.code,
                    e.message().to_string(),
                ),
            },
            diesel::result::Error::NotFound => {
                Self::Status(rocket::http::Status::NotFound.code, e.to_string())
            }
            diesel::result::Error::InvalidCString(_)
            | diesel::result::Error::QueryBuilderError(_)
            | diesel::result::Error::DeserializationError(_)
            | diesel::result::Error::SerializationError(_) => {
                Self::Status(rocket::http::Status::BadRequest.code, e.to_string())
            }
            e => Self::Database(e.to_string()),
        }
    }
}

impl From<AuthError> for Error {
    fn from(value: AuthError) -> Self {
        Error::Auth(value)
    }
}

#[cfg(feature = "rocket")]
impl From<rocket::http::Status> for Error {
    fn from(value: rocket::http::Status) -> Self {
        Error::Status(value.code, value.to_string())
    }
}

#[cfg(feature = "rocket")]
#[cfg(feature = "azure_core")]
impl From<azure_core::error::Error> for Error {
    fn from(value: azure_core::error::Error) -> Self {
        Error::Status(
            rocket::http::Status::InternalServerError.code,
            value.to_string(),
        )
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum UserError {
    NameTaken(String),
    NotFound(String),
    InvalidCredentials,
    NotConfirmed(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RegisterError {
    pub username_errors: Vec<String>,
    pub password_errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ValidationError {
    Country,
    Query(QueryValidationError),
    ResourceData(ResourceDataValidationError),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum UsernameValidationError {
    InvalidLength(i32, i32),
    InvalidCharacters(Vec<char>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum QueryValidationError {
    InvalidColumn(String),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ResourceDataValidationError {
    KeyMismatch(String, String),
    KeyMissing,
    ValueMissing,
}
