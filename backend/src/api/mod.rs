pub mod handler;
pub mod error;
pub mod resp;

pub type Result<T> = anyhow::Result<T, error::Error>;

use axum::{Router, Server};

use crate::config;
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

pub async fn run_server() -> anyhow::Result<()> {
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

    // information from global configuration
    let base = &config::global_config().base;
    let addr = format!("{}:{}", base.host, base.port).parse()?;

    // router
    let app = Router::new()
        .nest("/api", handler::router())
        .layer(request_id)
        .layer(timeout)
        .layer(compress);
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
