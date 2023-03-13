use anyhow::Result;
use backend::{api, cli, config, db};

#[tokio::main]
async fn main() -> Result<()> {
    let confpath = cli::execute().config;
    config::init_config(confpath)?;
    db::init_db().await?;
    tracing_subscriber::fmt()
        .json()
        .with_max_level(config::global_config().log.log_level())
        .with_current_span(true)
        .init();
    api::run_server().await?;
    Ok(())
}
