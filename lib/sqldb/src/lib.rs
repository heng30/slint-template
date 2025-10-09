//! SQL Database abstraction layer for Slint applications
//!
//! This library provides a simple SQLite database abstraction with async operations,
//! connection pooling, and common CRUD operations. It's designed to work seamlessly
//! with the Slint GUI framework across multiple platforms.
//!
//! # Features
//! - Async SQLite operations using `sqlx`
//! - Connection pooling with configurable limits
//! - Automatic database creation and table management
//! - Common data operations (insert, update, delete, select)
//! - Thread-safe operations with `tokio::sync::Mutex`
//! - Serde serialization support for data structures
//!
//! # Examples
//! ```no_run
//! use sqldb::{create_db, entry};
//! use anyhow::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create database
//!     create_db("/path/to/database.db").await?;
//!     
//!     // Create table
//!     entry::new("users").await?;
//!     
//!     // Insert data
//!     entry::insert("users", "user-123", "user data").await?;
//!     
//!     Ok(())
//! }
//! ```

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

/// Maximum number of concurrent database connections in the pool
const MAX_CONNECTIONS: u32 = 3;

/// Common database entry structure with UUID and data fields
///
/// This struct represents a generic database record that can be used
/// across different tables and applications. It provides serialization
/// support for both JSON and database operations.
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct ComEntry {
    /// Unique identifier for the entry
    pub uuid: String,
    /// Data payload stored as JSON string
    pub data: String,
}

/// Global database connection pool
///
/// This is a thread-safe connection pool that is lazily initialized
/// when the database is first created. It uses a mutex to ensure
/// safe concurrent access across async tasks.
static POOL: Lazy<Mutex<Option<Pool<Sqlite>>>> = Lazy::new(|| Mutex::new(None));

/// Get the global database connection pool
///
/// # Panics
/// Panics if the database pool has not been initialized.
/// Use `create_db()` to initialize the pool first.
async fn pool() -> Pool<Sqlite> {
    POOL.lock().await.clone().unwrap()
}

/// Create a new SQLite database and initialize the connection pool
///
/// This function creates the database file if it doesn't exist and
/// sets up a connection pool with the configured maximum connections.
///
/// # Arguments
/// * `db_path` - Path to the SQLite database file
///
/// # Errors
/// Returns an error if:
/// - The database cannot be created
/// - The connection pool cannot be established
///
/// # Example
/// ```no_run
/// use sqldb::create_db;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     create_db("/path/to/app.db").await?;
///     Ok(())
/// }
/// ```
pub async fn create_db(db_path: &str) -> Result<()> {
    Sqlite::create_database(db_path).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(&format!("sqlite:{}", db_path))
        .await?;

    *POOL.lock().await = Some(pool);

    Ok(())
}

/// Check if a table exists in the database
///
/// # Arguments
/// * `table_name` - Name of the table to check
///
/// # Returns
/// Returns `Ok(())` if the table exists, otherwise returns an error
///
/// # Errors
/// Returns an error if:
/// - The database query fails
/// - The table does not exist
pub async fn is_table_exist(table_name: &str) -> Result<()> {
    sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name=?")
        .bind(table_name)
        .fetch_one(&pool().await)
        .await?;

    Ok(())
}

/// Drop a table from the database
///
/// # Arguments
/// * `table_name` - Name of the table to drop
///
/// # Errors
/// Returns an error if:
/// - The table does not exist
/// - The database query fails
///
/// # Warning
/// This operation is destructive and cannot be undone.
/// Make sure to backup important data before calling this function.
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

    /// Initialize test database with a test table
    pub async fn init(db_path: &str) {
        create_db(db_path).await.expect("create db");
        entry::new("test").await.expect("account table failed");
    }

    /// Test database creation
    #[tokio::test]
    async fn test_create_db() -> Result<()> {
        let _mtx = MTX.lock().await;
        
        let test_db_path = "/tmp/test-create-db.db";
        
        // Clean up any existing test database
        let _ = std::fs::remove_file(test_db_path);
        
        create_db(test_db_path).await?;
        
        // Verify database file was created
        assert!(std::path::Path::new(test_db_path).exists());
        
        Ok(())
    }

    /// Test table existence checking
    #[tokio::test]
    async fn test_db_is_table_exist() -> Result<()> {
        let _mtx = MTX.lock().await;

        let test_db_path = "/tmp/test-is-table-exist.db";
        init(test_db_path).await;
        
        // Test non-existent table
        assert!(is_table_exist("hello").await.is_err());
        
        // Test existing table
        assert!(is_table_exist("test").await.is_ok());
        
        Ok(())
    }

    /// Test table dropping
    #[tokio::test]
    async fn test_db_drop_table() -> Result<()> {
        let _mtx = MTX.lock().await;

        let test_db_path = "/tmp/test-drop-table.db";
        init(test_db_path).await;
        
        // Test dropping non-existent table
        assert!(drop_table("hello").await.is_err());
        
        // Test dropping existing table
        assert!(drop_table("test").await.is_ok());
        
        // Verify table no longer exists
        assert!(is_table_exist("test").await.is_err());
        
        Ok(())
    }

    /// Test ComEntry struct serialization
    #[test]
    fn test_com_entry_serialization() {
        let entry = ComEntry {
            uuid: "test-uuid".to_string(),
            data: "test-data".to_string(),
        };

        // Test serialization to JSON
        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("test-uuid"));
        assert!(json.contains("test-data"));

        // Test deserialization from JSON
        let deserialized: ComEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.uuid, "test-uuid");
        assert_eq!(deserialized.data, "test-data");
    }

    /// Test ComEntry struct cloning
    #[test]
    fn test_com_entry_clone() {
        let original = ComEntry {
            uuid: "original-uuid".to_string(),
            data: "original-data".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(original.uuid, cloned.uuid);
        assert_eq!(original.data, cloned.data);
        
        // Verify they are separate instances
        assert!(!std::ptr::eq(&original, &cloned));
    }

    /// Test ComEntry struct debug formatting
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
