pub(crate) mod error;
pub(crate) mod resp;
#[macro_use]
pub(crate) mod v1;

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{MakeSpan, OnResponse, TraceLayer},
    ServiceBuilderExt,
};

pub(crate) type Result<T> = anyhow::Result<T, error::Error>;

pub(crate) fn router() -> Router {
    // https://docs.rs/tower-http/0.4.0/tower_http/request_id/index.html
    let request_id = ServiceBuilder::new()
        // make sure to set request ids before the request reaches `TraceLayer`
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(LogRequestId)
                .on_response(
                    LogResponse, // DefaultOnResponse::new()
                                 //     .level(tracing::Level::INFO)
                                 //     .latency_unit(tower_http::LatencyUnit::Millis),
                ),
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

const X_REQUEST_ID: &str = "x-request-id";

#[derive(Debug, Clone)]
struct LogRequestId;
impl<B> MakeSpan<B> for LogRequestId {
    fn make_span(&mut self, request: &axum::http::Request<B>) -> tracing::Span {
        let x_req_id = request
            .headers()
            .get(X_REQUEST_ID)
            .unwrap()
            .to_str()
            .unwrap();
        tracing::span!(tracing::Level::INFO, X_REQUEST_ID,
         "x-request-id" = %x_req_id,
        )
    }
}
#[derive(Debug, Clone)]
struct LogResponse;

impl<B> OnResponse<B> for LogResponse {
    fn on_response(
        self,
        response: &axum::http::Response<B>,
        latency: std::time::Duration,
        _span: &tracing::Span,
    ) {
        let status = response.status().as_u16();
        let latency = latency.as_millis();
        tracing::info!(
            "finished processing request with status {} in {} ms.",
            status,
            latency
        );
    }
}
