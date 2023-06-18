use crate::config::global_config;
use anyhow;
use once_cell::sync::OnceCell;
use sqlx::mysql::MySqlPoolOptions;

static DB_POOL: OnceCell<sqlx::MySqlPool> = OnceCell::new();

pub(crate) async fn init_database() -> anyhow::Result<()> {
    let config = global_config();
    let pool = MySqlPoolOptions::new().connect(&config.db.db_url()).await?;
    DB_POOL.set(pool).expect("set global database pool failed.");
    tracing::info!("database connected.");
    Ok(())
}

pub(crate) fn db_pool() -> &'static sqlx::MySqlPool {
    DB_POOL.get().expect("get global database pool failed.")
}
