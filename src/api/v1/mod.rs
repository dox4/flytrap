use axum::Router;
pub(crate) mod request;

pub(crate) fn router() -> Router {
    Router::new().nest("/request", request::router())
}

trait UpdateWith<T: Sized> {
    fn update_with(self, other: T) -> Self;
}
