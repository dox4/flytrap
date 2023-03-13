use crate::api::resp;
use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("resource not found.")]
    NotFound,
    #[error("parse uuid failed: {0}")]
    BadUUID(#[from] uuid::Error),
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl Error {
    fn status(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::BadUUID(_) => 400,
            _ => 500,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        resp::Response::error(self.status(), self).into_response()
    }
}
