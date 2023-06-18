use super::error::Error;
use super::Result;
use axum::{body, response::IntoResponse, Json};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub(crate) struct Created {
    pub(crate) id: Uuid,
}

impl Into<Created> for Uuid {
    fn into(self) -> Created {
        Created { id: self }
    }
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

#[derive(Debug, Serialize)]
pub(crate) struct BatchCreated {
    pub(crate) ids: Vec<Uuid>,
}

impl Into<BatchCreated> for Vec<Uuid> {
    fn into(self) -> BatchCreated {
        BatchCreated { ids: self }
    }
}

impl IntoResponse for BatchCreated {
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
pub(crate) struct FetchOne<T: Serialize> {
    pub(crate) data: T,
}

impl<T> FetchOne<T>
where
    T: Serialize,
{
    pub(crate) fn new(data: T) -> Self {
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
pub(crate) struct FetchPaged<T: Serialize> {
    pub(crate) total: i64,
    pub(crate) data: Vec<T>,
}

impl<T> FetchPaged<T>
where
    T: Serialize,
{
    pub(crate) fn new(total: i64, data: Vec<T>) -> Self {
        Self { total, data }
    }

    pub(crate) fn empty() -> Self {
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
pub(crate) struct RowsAffected {
    pub(crate) rows_affected: u64,
}

impl Into<RowsAffected> for u64 {
    fn into(self) -> RowsAffected {
        RowsAffected {
            rows_affected: self,
        }
    }
}

pub(crate) trait ExpectRowsAffected {
    fn expect(self, expect: u64) -> Result<RowsAffected>
    where
        Self: Sized;
}

impl ExpectRowsAffected for u64 {
    fn expect(self, expect: u64) -> Result<RowsAffected> {
        if self != expect {
            Err(Error::UnexpectedRowsAffected(expect, self))
        } else {
            Ok(self.into())
        }
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
pub(crate) struct Response {
    #[serde(skip_serializing_if = "is_ok")]
    pub(crate) status: u16,
    pub(crate) message: String,
}

impl Response {
    // pub(crate) fn ok<M: ToString>(message: M) -> Self {
    //     Self {
    //         status: 200,
    //         message: message.to_string(),
    //     }
    // }

    pub(crate) fn error<M: ToString>(status: u16, message: M) -> Self {
        Self {
            status,
            message: message.to_string(),
        }
    }

    // pub(crate) fn bad_request<M: ToString>(message: M) -> Self {
    //     Self::error(400, message)
    // }

    // pub(crate) fn not_found<M: ToString>(message: M) -> Self {
    //     Self::error(404, message)
    // }

    // pub(crate) fn unauthorized<M: ToString>(message: M) -> Self {
    //     Self::error(401, message)
    // }

    // pub(crate) fn forbidden<M: ToString>(message: M) -> Self {
    //     Self::error(403, message)
    // }

    // pub(crate) fn internal<M: ToString>(message: M) -> Self {
    //     Self::error(500, message)
    // }
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
