use crate::api::resp::FetchOne;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_crud::{add_timed_fields, SqlxCrud};
use uuid::Uuid;

// HTTP Request
// +----------+----------------------+-----------+
// | <Method> | <Query>              | <Version> |
// | POST     | /api?name=foo&age=20 | HTTP/1.1  |
// +-------------+----------------------+--------+
// | <Headers>                                   |
// | Host: localhost:8080                        |
// | Content-Type: application/json              |
// | Content-Length: 17                          |
// +---------------------------------------------+
// | <Blank Line>                                |
// +---------------------------------------------+
// | <Body>                                      |
// | { "bar": "okay" }                           |
// +---------------------------------------------+

#[add_timed_fields]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, SqlxCrud, Default)]
pub(crate) struct Request {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) method: String,
    pub(crate) path: String,
    pub(crate) query: serde_json::Value,
    pub(crate) host: String,
    pub(crate) headers: serde_json::Value,
    pub(crate) body: Option<serde_json::Value>,
}

impl IntoResponse for Request {
    fn into_response(self) -> axum::response::Response {
        FetchOne::new(self).into_response()
    }
}
