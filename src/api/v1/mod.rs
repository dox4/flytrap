use axum::Router;
pub(crate) mod execution;
pub(crate) mod request;

pub(crate) fn router() -> Router {
    Router::new()
        .nest("/request", request::router())
        .nest("/execution", execution::router())
}

trait UpdateWith<T: Sized> {
    fn update_with(self, other: T) -> Self;
}

trait QueryWith<T: Sized> {
    fn query_with(self, query: &mut sql_builder::SqlBuilder);
}
