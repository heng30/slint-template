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
    sqlx::query(sqlx::AssertSqlSafe(format!("DROP TABLE {}", table_name)))
        .execute(&pool().await)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    pub async fn init(db_path: &str) {
        create_db(db_path).await.expect("create db");
        entry::new("test").await.expect("account table failed");
    }

    #[tokio::test]
    async fn test_create_db() -> Result<()> {
        let _mtx = MTX.lock().await;

        let test_db_path = "/tmp/test-create-db.db";

        let _ = std::fs::remove_file(test_db_path);

        create_db(test_db_path).await?;

        assert!(std::path::Path::new(test_db_path).exists());

        Ok(())
    }

    #[tokio::test]
    async fn test_db_is_table_exist() -> Result<()> {
        let _mtx = MTX.lock().await;

        let test_db_path = "/tmp/test-is-table-exist.db";
        init(test_db_path).await;

        assert!(is_table_exist("hello").await.is_err());

        assert!(is_table_exist("test").await.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_db_drop_table() -> Result<()> {
        let _mtx = MTX.lock().await;

        let test_db_path = "/tmp/test-drop-table.db";
        init(test_db_path).await;

        assert!(drop_table("hello").await.is_err());

        assert!(drop_table("test").await.is_ok());

        assert!(is_table_exist("test").await.is_err());

        Ok(())
    }

    #[test]
    fn test_com_entry_serialization() {
        let entry = ComEntry {
            uuid: "test-uuid".to_string(),
            data: "test-data".to_string(),
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("test-uuid"));
        assert!(json.contains("test-data"));

        let deserialized: ComEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.uuid, "test-uuid");
        assert_eq!(deserialized.data, "test-data");
    }

    #[test]
    fn test_com_entry_clone() {
        let original = ComEntry {
            uuid: "original-uuid".to_string(),
            data: "original-data".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(original.uuid, cloned.uuid);
        assert_eq!(original.data, cloned.data);

        assert!(!std::ptr::eq(&original, &cloned));
    }

    #[test]
    fn test_com_entry_debug() {
        let entry = ComEntry {
            uuid: "debug-uuid".to_string(),
            data: "debug-data".to_string(),
        };

        let debug_output = format!("{:?}", entry);
        assert!(debug_output.contains("debug-uuid"));
        assert!(debug_output.contains("debug-data"));
    }
}
