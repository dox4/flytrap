use crate::api::resp;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected rows affected, expected {0}, got {1}")]
    UnexpectedRowsAffected(u64, u64),
    #[error("resource not found.")]
    NotFound,
    #[error("resource created failed: {0}")]
    CreateFailed(String),
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl Error {
    fn status(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            _ => 500,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        resp::Response::error(self.status(), self).into_response()
    }
}
