use crate::{api, config, db, log};
use anyhow::Result;
use axum::Server;
use clap::Parser;

const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

const fn authors() -> &'static str {
    env!("CARGO_PKG_AUTHORS")
}
const fn about() -> &'static str {
    env!("CARGO_PKG_DESCRIPTION")
}

#[derive(Parser)]
#[clap(
    version = version(),
    author = authors(),
    about = about(),
)]
pub(crate) enum App {
    #[clap(
        name = "dump-default-config",
        about = "dump the default config, do not use it in production."
    )]
    Dump,
    #[clap(name = "serve", about = "run the server with the given config file.")]
    Serve {
        #[clap(
            long = "config-file",
            value_name = "FILE",
            help = "set a custom config file"
        )]
        config_file: Option<String>,
    },
}

impl App {
    async fn execute(&self) -> Result<()> {
        match self {
            App::Dump => Self::dump_default_config().await,
            App::Serve { config_file } => {
                config::init_config(config_file)?;
                let _ = log::init_log().await?;
                db::init_database().await?;
                Self::serve().await
            }
        }
    }

    async fn dump_default_config() -> Result<()> {
        let conf = config::Config::default();
        let data = toml::to_string_pretty(&conf)?;
        println!("{}", data);
        Ok(())
    }
    async fn serve() -> Result<()> {
        // information from global configuration
        let base = &config::global_config().base;
        let addr = format!("{}:{}", base.host, base.port).parse()?;
        let app = api::router();
        tracing::info!("listening on {}", addr);
        Server::bind(&addr).serve(app.into_make_service()).await?;
        Ok(())
    }
}

pub async fn run() -> Result<()> {
    App::parse().execute().await
}
