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

// async fn retrieve_list(Query(request): Query<RequestQuery>) -> Result<FetchPaged<Request>> {
//     let page = request.page.unwrap_or(1);
//     let per_page = request.per_page.unwrap_or(10);
//     let offset = (page - 1) * per_page;
//     let mut query = sql_builder::SqlBuilder::select_from(Request::table_name());
//     query.offset(offset).limit(page);
//     request.query_with(&mut query);
//     let count_sql = query.clone().count("0").sql().unwrap();
//     let count: i64 = sqlx::query(&count_sql)
//         .fetch_one(db::db_pool())
//         .await?
//         .get(0);
//     let data_sql = query.sql().unwrap();
//     let list = sqlx::query_as::<_, Request>(&data_sql)
//         .fetch_all(db::db_pool())
//         .await?;
//     Ok((count, list).into())
// }

router!();
create!(RequestRequest, Request);
retrieve!(Request);
retrieve_list!(RequestQuery, Request);
update!(RequestRequest, Request);
delete!(Request);
