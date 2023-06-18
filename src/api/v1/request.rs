use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx_crud::Crud;
use uuid::Uuid;

use crate::{
    api::{
        resp::{ExpectRowsAffected, RowsAffected},
        Result,
    },
    create, db, delete,
    entity::request::Request,
    retrieve, router, update,
};

use super::UpdateWith;

#[derive(Debug, Deserialize)]
struct RequestRequest {
    name: String,
    method: String,
    path: String,
    query: serde_json::Value,
    host: String,
    headers: serde_json::Value,
    body: Option<serde_json::Value>,
}

impl Into<Request> for RequestRequest {
    fn into(self) -> Request {
        Request {
            id: uuid::Uuid::new_v4(),
            name: self.name,
            method: self.method,
            path: self.path,
            query: self.query,
            host: self.host,
            headers: self.headers,
            body: self.body,
            ..Default::default()
        }
    }
}

impl UpdateWith<RequestRequest> for Request {
    fn update_with(mut self, request: RequestRequest) -> Request {
        self.name = request.name;
        self.method = request.method;
        self.path = request.path;
        self.query = request.query;
        self.host = request.host;
        self.headers = request.headers;
        self.body = request.body;
        self
    }
}

// router and crud macros are defined in src/common/macros.rs
router!();
create!(RequestRequest, Request);
retrieve!(Request);
update!(RequestRequest, Request);
delete!(Request);
