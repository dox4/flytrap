use crate::api::resp::FetchOne;
use axum::response::IntoResponse;
use chrono::{DateTime, Local};
use serde::Serialize;
use serde_json::Value;
use sqlx::FromRow;
use sqlx_crud::SqlxCrud;
use uuid::fmt::Hyphenated;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromRow, SqlxCrud)]
pub(crate) struct RawHttpRequest {
    pub(crate) id: u64,
    pub(crate) method: String,
    pub(crate) url: String,
    pub(crate) version: String,
    pub(crate) headers: Value,
    pub(crate) body: Option<Value>,
}

impl IntoResponse for RawHttpRequest {
    fn into_response(self) -> axum::response::Response {
        FetchOne::new(self).into_response()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, FromRow, SqlxCrud)]
pub(crate) struct RawHttpResponse {
    pub(crate) id: u64,
    pub(crate) version: String,
    pub(crate) status_code: u16,
    pub(crate) status_message: String,
    pub(crate) headers: Value,
    pub(crate) body: Value,
}

impl IntoResponse for RawHttpResponse {
    fn into_response(self) -> axum::response::Response {
        FetchOne::new(self).into_response()
    }
}

#[derive(Debug, Clone, Serialize, FromRow, SqlxCrud, Default)]
pub(crate) struct Execution {
    pub(crate) id: Hyphenated,
    pub(crate) request: u64,
    pub(crate) request_time: DateTime<Local>,
    pub(crate) response_time: DateTime<Local>,
    pub(crate) response: u64,
}

impl IntoResponse for Execution {
    fn into_response(self) -> axum::response::Response {
        FetchOne::new(self).into_response()
    }
}
