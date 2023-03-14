use axum::Json;
use axum::Router;
use chrono::Local;
use sql_builder::update::UpdateQuery;
use sql_builder::where_clause::WhereClause;
use sql_builder::SqlBuilder;
use traits::Schema;

use crate::api::error;
use crate::api::req::DeleteByIds;
use crate::api::resp::RowsAffected;
use crate::api::Result;
use crate::db;

pub mod request;

pub fn router() -> Router {
    Router::new().nest("/request", request::router())
}

pub async fn delete_by_ids<T: Schema>(Json(arg): Json<DeleteByIds>) -> Result<RowsAffected> {
    let sql = UpdateQuery::new(T::table_name())
        .set_field("deleted_at", &Local::now())
        .add_where_clause(WhereClause::in_("id", &arg.ids))
        .build()
        .unwrap();
    tracing::debug!("sql: {}", sql);
    let cnt = sqlx::query(&sql)
        .execute(db::db_pool())
        .await
        .map_err(|e| error::Error::DatabaseError(e))?
        .rows_affected();
    if cnt != arg.ids.len() as u64 {
        return Err(error::Error::UnexpectedRowsAffected(
            arg.ids.len() as u64,
            cnt,
        ));
    }
    Ok(RowsAffected::new(cnt))
}
