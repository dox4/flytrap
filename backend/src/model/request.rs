use crate::api::resp::FetchOne;
use crate::datatype::DataId;
use axum::response::IntoResponse;
use basemodel::with_basemodel;
use basemodel::BaseModel;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use traits::Schema;

// HTTP Request
// Method Path Query Body Headers Host
// GET /api?name=foo&age=20 HTTP/1.1
// Host: localhost:8080
// Content-Type: application/json
// Content-Length: 20
//
// {}

#[with_basemodel]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, BaseModel)]
pub struct Request {
    pub id: DataId,
    pub name: String,
    pub method: String,
    pub path: String,
    pub query: serde_json::Value,
    pub host: String,
    pub headers: serde_json::Value,
    pub body: Option<serde_json::Value>,
}

impl IntoResponse for Request {
    fn into_response(self) -> axum::response::Response {
        FetchOne::new(self).into_response()
    }
}
