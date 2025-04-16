use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(DieselError),
    Env(std::env::VarError),
    Pool(r2d2::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Database(err) => write!(f, "Database error: {}", err),
            AppError::Env(err) => write!(f, "Environment error: {}", err),
            AppError::Pool(err) => write!(f, "Database pool error: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<DieselError> for AppError {
    fn from(err: DieselError) -> Self {
        AppError::Database(err)
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::Env(err)
    }
}

impl From<r2d2::Error> for AppError {
    fn from(err: r2d2::Error) -> Self {
        AppError::Pool(err)
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(err) => match err {
                DieselError::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => HttpResponse::BadRequest().body("Unique constraint violation"),
                _ => HttpResponse::InternalServerError().body("Database error"),
            },
            AppError::Env(_) => HttpResponse::InternalServerError().body("Environment error"),
            AppError::Pool(_) => HttpResponse::InternalServerError().body("Database pool error"),
        }
    }
}
