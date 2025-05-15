use crate::server::error::api_error::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("{0}")]
    SomethingWentWrong(String),
    #[error("Duplicate entry exists")]
    UniqueConstraintViolation(String),
    #[error("Record not found")]
    RecordNotFound(String),
    #[error("No fields to update")]
    NoFieldsToUpdate,
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let status_code = match self {
            DbError::SomethingWentWrong(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::UniqueConstraintViolation(_) => StatusCode::CONFLICT,
            DbError::RecordNotFound(_) => StatusCode::NOT_FOUND,
            DbError::NoFieldsToUpdate => StatusCode::BAD_REQUEST,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
