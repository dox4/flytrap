use crate::api::resp::FetchOne;
use axum::response::IntoResponse;
use serde::Serialize;
use serde_json::Value;
use sqlx::types::uuid::fmt::Hyphenated;
use sqlx::FromRow;
use sqlx_crud::{add_timed_fields, SqlxCrud};

#[add_timed_fields]
#[derive(Debug, Clone, Serialize, FromRow, SqlxCrud, Default)]
pub(crate) struct Response {
    pub(crate) id: Hyphenated,
    pub(crate) status: u16,
    pub(crate) header: Value,
    pub(crate) body: Option<Value>,
    pub(crate) message: Option<String>, // for the failed request that response raw message
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        FetchOne::new(self).into_response()
    }
}
