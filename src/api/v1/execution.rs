use chrono::Local;

use chrono::DateTime;

use axum::{
    extract::{Path, Query},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde::Serialize;
use sqlx::Row;
use sqlx_crud::{Crud, Schema};
use uuid::fmt::Hyphenated;
use uuid::Uuid;

use crate::entity::execution::RawHttpRequest;
use crate::entity::execution::RawHttpResponse;
use crate::{
    api::{
        error,
        resp::{ExpectRowsAffected, FetchPaged, RowsAffected},
        Result,
    },
    db, delete, retrieve, retrieve_list, router, service,
};

use super::QueryWith;

use crate::entity::execution::Execution;

#[derive(Debug, Clone, Deserialize)]
struct ExecutionRequest {
    request_id: Uuid,
}

impl Into<Execution> for ExecutionRequest {
    fn into(self) -> Execution {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ExecutionQuery {
    page: Option<usize>,
    per_page: Option<usize>,
    request_id: Uuid,
}

impl QueryWith<Execution> for ExecutionQuery {
    fn query_with(self, query: &mut sql_builder::SqlBuilder) {
        query.and_where_eq("request_id", format!("'{}'", self.request_id));
    }
}

router!();

#[derive(Debug, Serialize)]
struct ExecutionRecord {
    pub(crate) id: Hyphenated,
    pub(crate) request: RawHttpRequest,
    pub(crate) request_time: DateTime<Local>,
    pub(crate) response_time: DateTime<Local>,
    pub(crate) response: RawHttpResponse,
}

async fn create(Json(arg): Json<ExecutionRequest>) -> Result<ExecutionRecord> {
    let request_id = arg.request_id.hyphenated();
    let execution = service::execution::execute_request(request_id)
        .await
        .map_err(|e| error::Error::CreateFailed(e.to_string()))?;
    Ok(ExecutionRecord {
        id: execution.id,
        request: RawHttpRequest::by_id(db::db_pool(), execution.request)
            .await?
            .unwrap(),
        request_time: execution.request_time,
        response_time: execution.response_time,
        response: RawHttpResponse::by_id(db::db_pool(), execution.response)
            .await?
            .unwrap(),
    })
}
retrieve!(Execution);
retrieve_list!(ExecutionQuery, Execution);
delete!(Execution);

async fn update() -> Result<()> {
    unimplemented!()
}
