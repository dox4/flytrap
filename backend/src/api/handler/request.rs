use crate::api::resp::{Created, FetchPaged};
use crate::api::Result;
use crate::{db, model::request::Request};
use axum::extract::Query;
use axum::routing::{delete, put};
use axum::Json;
use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use sql_builder::update::UpdateQuery;
use sql_builder::{
    insert::InsertQuery, repr::ToSqlRepr, select::SelectQuery, where_clause::WhereClause,
    SqlBuilder,
};

use traits::Schema;
use uuid::Uuid;

use super::delete_by_ids;

pub fn router() -> Router {
    Router::new()
        .route("/", post(make_request))
        .route("/", get(fetch_request_paged))
        .route("/", delete(delete_by_ids::<Request>))
        .route("/:id", get(fetch_request_by_id))
        .route("/:id", put(update_request_by_id))
}

#[derive(Debug, Deserialize)]
pub struct CreateOrUpdateRequerstArgument {
    pub name: String,
    pub method: String,
    pub path: String,
    pub query: serde_json::Value,
    pub host: String,
    pub headers: serde_json::Value,
    pub body: Option<serde_json::Value>,
}

impl From<CreateOrUpdateRequerstArgument> for Request {
    fn from(arg: CreateOrUpdateRequerstArgument) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: arg.name,
            method: arg.method,
            path: arg.path,
            query: arg.query,
            host: arg.host,
            headers: arg.headers,
            body: arg.body,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}

async fn make_request(Json(arg): Json<CreateOrUpdateRequerstArgument>) -> Result<Created> {
    let u = uuid::Uuid::new_v4();
    let sql = InsertQuery::new(Request::table_name())
        .add_columns(
            [
                "id", "name", "method", "path", "query", "host", "headers", "body",
            ]
            .into_iter(),
        )
        .add_record_raw(&[
            u.to_sql_repr(),
            arg.name.to_sql_repr(),
            arg.method.to_sql_repr(),
            arg.path.to_sql_repr(),
            arg.query.to_sql_repr(),
            arg.host.to_sql_repr(),
            arg.headers.to_sql_repr(),
            arg.body.to_sql_repr(),
        ])
        .unwrap()
        .build()
        .unwrap();
    db::insert_one(&sql).await?;
    Ok(Created { id: u })
}

pub async fn fetch_request_by_id(Path(id): Path<Uuid>) -> Result<Request> {
    let sql = SelectQuery::new(Request::table_name())
        .add_columns(&Request::column_names())
        .where_clause(WhereClause::equals("id", id).and_is_null("deleted_at"))
        .build()
        .unwrap();
    db::fetch_one::<Request>(&sql).await
}

#[derive(Debug, Deserialize)]
pub struct PageArg {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

impl PageArg {
    fn paged(&self) -> (u64, u64) {
        (self.page.unwrap_or(1), self.per_page.unwrap_or(10))
    }
}

pub async fn fetch_request_paged(Query(arg): Query<PageArg>) -> Result<FetchPaged<Request>> {
    let count_sql = SelectQuery::new(Request::table_name())
        .add_columns(&["count(*)"])
        .where_clause(WhereClause::is_null("deleted_at"))
        .build()
        .unwrap();
    let count = db::fetch_count(&count_sql).await?;
    if count == 0 {
        return Ok(FetchPaged::empty());
    }
    let (page, per_page) = arg.paged();
    tracing::debug!("page: {}, per_page: {}", page, per_page);
    let sql = SelectQuery::new(Request::table_name())
        .add_columns(Request::column_names())
        .where_clause(WhereClause::is_null("deleted_at"))
        .limit(per_page)
        .offset((page - 1) * per_page)
        .build()
        .unwrap();
    let data = db::fetch_all::<Request>(&sql).await?;
    Ok(FetchPaged::new(count, data))
}

pub async fn update_request_by_id(
    Path(id): Path<Uuid>,
    Json(arg): Json<CreateOrUpdateRequerstArgument>,
) -> Result<Request> {
    let req = Request::from(arg);
    let sql = UpdateQuery::new(Request::table_name())
        .set_field("name", &req.name)
        .set_field("method", &req.method)
        .set_field("path", &req.path)
        .set_field("query", &req.query)
        .set_field("host", &req.host)
        .set_field("headers", &req.headers)
        .set_field("body", &req.body)
        .add_where_clause(WhereClause::equals("id", id).and_is_null("deleted_at"))
        .build()
        .unwrap();
    db::update_one(&sql).await?;
    fetch_request_by_id(Path(id)).await
}
