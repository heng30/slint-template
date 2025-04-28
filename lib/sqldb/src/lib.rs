use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{Sqlite, SqlitePoolOptions},
    Pool,
};
use tokio::sync::Mutex;

pub mod entry;

const MAX_CONNECTIONS: u32 = 3;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct ComEntry {
    pub uuid: String,
    pub data: String,
}

static POOL: Lazy<Mutex<Option<Pool<Sqlite>>>> = Lazy::new(|| Mutex::new(None));

async fn pool() -> Pool<Sqlite> {
    POOL.lock().await.clone().unwrap()
}

pub async fn create_db(db_path: &str) -> Result<()> {
    Sqlite::create_database(db_path).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(&format!("sqlite:{}", db_path))
        .await?;

    *POOL.lock().await = Some(pool);

    Ok(())
}

pub async fn is_table_exist(table_name: &str) -> Result<()> {
    sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
        .bind(table_name)
        .fetch_one(&pool().await)
        .await?;

    Ok(())
}

pub async fn drop_table(table_name: &str) -> Result<()> {
    sqlx::query(&format!("DROP TABLE {}", table_name))
        .execute(&pool().await)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
    const DB_PATH: &str = "/tmp/db-test.db";

    pub async fn init(db_path: &str) {
        create_db(db_path).await.expect("create db");
        entry::new("test").await.expect("account table failed");
    }

    #[tokio::test]
    async fn test_db_is_table_exist() -> Result<()> {
        let _mtx = MTX.lock().await;

        init(DB_PATH).await;
        assert!(is_table_exist("hello").await.is_err());
        assert!(is_table_exist("test").await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_db_drop_table() -> Result<()> {
        let _mtx = MTX.lock().await;

        init(DB_PATH).await;
        assert!(drop_table("hello").await.is_err());
        assert!(drop_table("test").await.is_ok());
        Ok(())
    }
}
