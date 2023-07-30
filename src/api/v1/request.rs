use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use sqlx::Row;
use sqlx_crud::{Crud, Schema};
use uuid::Uuid;

use crate::{
    api::{
        resp::{ExpectRowsAffected, FetchPaged, RowsAffected},
        Result,
    },
    create, db, delete,
    entity::request::Request,
    retrieve, retrieve_list, router, update,
};

use super::{QueryWith, UpdateWith};

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
            id: uuid::Uuid::new_v4().hyphenated(),
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

#[derive(Debug, Deserialize)]
pub(crate) struct RequestQuery {
    pub(crate) name: Option<String>,
    pub(crate) page: Option<usize>,
    pub(crate) per_page: Option<usize>,
}

impl QueryWith<Request> for RequestQuery {
    fn query_with(self, query: &mut sql_builder::SqlBuilder) {
        if let Some(ref name) = self.name {
            query.and_where_like("name", format!("%{}%", name));
        }
    }
}

router!();
create!(RequestRequest, Request);
retrieve!(Request);
retrieve_list!(RequestQuery, Request);
update!(RequestRequest, Request);
delete!(Request);
