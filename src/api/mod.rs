pub(crate) mod error;
pub(crate) mod resp;
#[macro_use]
pub(crate) mod v1;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

pub(crate) type Result<T> = anyhow::Result<T, error::Error>;

pub(crate) fn router() -> Router {
    // layers
    let mru = MakeRequestUuid {};
    // here I use the code from the tower-http example:
    // https://docs.rs/tower-http/0.4.0/tower_http/request_id/index.html
    let request_id = ServiceBuilder::new()
        // make sure to set request ids before the request reaches `TraceLayer`
        .set_x_request_id(mru)
        .layer(
            TraceLayer::new_for_http()
                // TODO: simplify layer to only log x-request-id, rather than all headers
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true)),
        )
        // propagate the header to the response before the response reaches `TraceLayer`
        .propagate_x_request_id();
    let timeout =
        ServiceBuilder::new().layer(TimeoutLayer::new(std::time::Duration::from_secs(10)));
    let compress = ServiceBuilder::new().layer(tower_http::compression::CompressionLayer::new());

    // router
    Router::new()
        .nest("/api/v1", v1::router())
        .layer(request_id)
        .layer(timeout)
        .layer(compress)
}
