use axum::Router;

pub mod request;

pub fn router() -> Router {
    Router::new().nest("/request", request::router())
}
