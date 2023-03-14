use crate::api::{error, Result};
use crate::config::global_config;
use anyhow;
use once_cell::sync::OnceCell;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::FromRow;

static DB_POOL: OnceCell<sqlx::MySqlPool> = OnceCell::new();

pub async fn init_db() -> anyhow::Result<(), sqlx::Error> {
    let config = global_config();
    let pool = MySqlPoolOptions::new().connect(&config.db.db_url()).await?;
    DB_POOL
        .set(pool)
        .expect("error occurred when setting db pool.");
    Ok(())
}

pub fn db_pool() -> &'static sqlx::MySqlPool {
    DB_POOL.get().expect("db pool not initialized.")
}

pub async fn insert_one(sql: &str) -> Result<()> {
    sqlx::query(sql)
        .execute(db_pool())
        .await
        .map_err(|e| error::Error::DatabaseError(e))?;
    Ok(())
}

pub async fn fetch_one<O>(sql: &str) -> Result<O>
where
    O: Send,
    O: Unpin,
    O: for<'r> FromRow<'r, sqlx::mysql::MySqlRow>,
{
    Ok(sqlx::query_as::<_, O>(&sql)
        .fetch_one(db_pool())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => error::Error::NotFound,
            _ => error::Error::DatabaseError(e),
        })?)
}

pub async fn fetch_count(sql: &str) -> Result<i64> {
    Ok(sqlx::query_as::<_, (i64,)>(sql)
        .fetch_one(db_pool())
        .await
        .map_err(|e| error::Error::DatabaseError(e))?
        .0)
}

pub async fn fetch_all<O>(sql: &str) -> Result<Vec<O>>
where
    O: Send,
    O: Unpin,
    O: for<'r> FromRow<'r, sqlx::mysql::MySqlRow>,
{
    Ok(sqlx::query_as::<_, O>(&sql)
        .fetch_all(db_pool())
        .await
        .map_err(|e| error::Error::DatabaseError(e))?)
}

pub async fn update_one(sql: &str) -> Result<()> {
    let cnt = sqlx::query(sql)
        .execute(db_pool())
        .await
        .map_err(|e| error::Error::DatabaseError(e))?
        .last_insert_id();
    if cnt > 1 {
        return Err(error::Error::UnexpectedRowsAffected(1, cnt));
    }
    Ok(())
}
