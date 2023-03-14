use axum::{body, response::IntoResponse, Json};
use serde::Serialize;

use crate::datatype::DataId;

#[derive(Debug, Serialize)]
pub struct Created {
    pub id: DataId,
}

impl IntoResponse for Created {
    fn into_response(self) -> axum::response::Response {
        let body = body::boxed(body::Full::from(serde_json::to_string(&self).unwrap()));
        axum::response::Response::builder()
            .status(201)
            .body(body)
            .unwrap()
    }
}

// define the response for fetching one resource
#[derive(Debug, Serialize)]
pub struct FetchOne<T: Serialize> {
    pub data: T,
}

impl<T> FetchOne<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> IntoResponse for FetchOne<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

// define the response for fetching paged list resource
#[derive(Debug, Serialize)]
pub struct FetchPaged<T: Serialize> {
    pub total: i64,
    pub data: Vec<T>,
}

impl<T> FetchPaged<T>
where
    T: Serialize,
{
    pub fn new(total: i64, data: Vec<T>) -> Self {
        Self { total, data }
    }

    pub fn empty() -> Self {
        Self {
            total: 0,
            data: Vec::new(),
        }
    }
}

impl<T> IntoResponse for FetchPaged<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct RowsAffected {
    pub rows_affected: u64,
}

impl RowsAffected {
    pub fn new(rows_affected: u64) -> Self {
        Self { rows_affected }
    }
}

impl IntoResponse for RowsAffected {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

fn is_ok(status: &u16) -> bool {
    *status == 200
}

// define the universal response
#[derive(Debug, Clone, Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "is_ok")]
    pub status: u16,
    pub message: String,
}

impl Response {
    pub fn ok<M: ToString>(message: M) -> Self {
        Self {
            status: 200,
            message: message.to_string(),
        }
    }

    pub fn error<M: ToString>(status: u16, message: M) -> Self {
        Self {
            status,
            message: message.to_string(),
            // data: None,
        }
    }

    pub fn bad_request<M: ToString>(message: M) -> Self {
        Self::error(400, message)
    }

    pub fn not_found<M: ToString>(message: M) -> Self {
        Self::error(404, message)
    }

    pub fn unauthorized<M: ToString>(message: M) -> Self {
        Self::error(401, message)
    }

    pub fn forbidden<M: ToString>(message: M) -> Self {
        Self::error(403, message)
    }

    pub fn internal<M: ToString>(message: M) -> Self {
        Self::error(500, message)
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let body = body::boxed(body::Full::from(serde_json::to_string(&self).unwrap()));
        axum::response::Response::builder()
            .status(self.status)
            .body(body)
            .unwrap()
    }
}
