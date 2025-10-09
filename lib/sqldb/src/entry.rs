//! Database table operations for managing ComEntry records
//!
//! This module provides CRUD (Create, Read, Update, Delete) operations
//! for database tables that store `ComEntry` records. All operations
//! are async and use the global connection pool.

use super::{ComEntry, pool};
use anyhow::Result;

/// Create a new table for storing ComEntry records
///
/// This function creates a table with the following schema:
/// - `id`: INTEGER PRIMARY KEY (auto-incrementing)
/// - `uuid`: TEXT NOT NULL UNIQUE (unique identifier)
/// - `data`: TEXT NOT NULL (data payload)
///
/// # Arguments
/// * `table` - Name of the table to create
///
/// # Errors
/// Returns an error if:
/// - The table creation query fails
/// - The database connection is not available
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     entry::new("users").await?;
///     Ok(())
/// }
/// ```
pub async fn new(table: &str) -> Result<()> {
    sqlx::query(&format!(
        "CREATE TABLE IF NOT EXISTS {table} (
             id INTEGER PRIMARY KEY,
             uuid TEXT NOT NULL UNIQUE,
             data TEXT NOT NULL
             )"
    ))
    .execute(&pool().await)
    .await?;

    Ok(())
}

/// Delete a specific entry from the table by UUID
///
/// # Arguments
/// * `table` - Name of the table
/// * `uuid` - Unique identifier of the entry to delete
///
/// # Errors
/// Returns an error if:
/// - The entry does not exist
/// - The database query fails
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     entry::delete("users", "user-123").await?;
///     Ok(())
/// }
/// ```
pub async fn delete(table: &str, uuid: &str) -> Result<()> {
    sqlx::query(&format!("DELETE FROM {table} WHERE uuid=?"))
        .bind(uuid)
        .execute(&pool().await)
        .await?;
    Ok(())
}

/// Delete all entries from the table
///
/// # Arguments
/// * `table` - Name of the table
///
/// # Errors
/// Returns an error if the database query fails
///
/// # Warning
/// This operation removes all data from the table and cannot be undone.
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     entry::delete_all("users").await?;
///     Ok(())
/// }
/// ```
pub async fn delete_all(table: &str) -> Result<()> {
    sqlx::query(&format!("DELETE FROM {table}"))
        .execute(&pool().await)
        .await?;
    Ok(())
}

/// Insert a new entry into the table
///
/// # Arguments
/// * `table` - Name of the table
/// * `uuid` - Unique identifier for the new entry
/// * `data` - Data payload to store
///
/// # Errors
/// Returns an error if:
/// - An entry with the same UUID already exists (violates UNIQUE constraint)
/// - The database query fails
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     entry::insert("users", "user-123", "user data").await?;
///     Ok(())
/// }
/// ```
pub async fn insert(table: &str, uuid: &str, data: &str) -> Result<()> {
    sqlx::query(&format!("INSERT INTO {table} (uuid, data) VALUES (?, ?)"))
        .bind(uuid)
        .bind(data)
        .execute(&pool().await)
        .await?;
    Ok(())
}

/// Update an existing entry in the table
///
/// # Arguments
/// * `table` - Name of the table
/// * `uuid` - Unique identifier of the entry to update
/// * `data` - New data payload
///
/// # Errors
/// Returns an error if:
/// - The entry does not exist
/// - The database query fails
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     entry::update("users", "user-123", "updated data").await?;
///     Ok(())
/// }
/// ```
pub async fn update(table: &str, uuid: &str, data: &str) -> Result<()> {
    sqlx::query(&format!("UPDATE {table} SET data=? WHERE uuid=?"))
        .bind(data)
        .bind(uuid)
        .execute(&pool().await)
        .await?;

    Ok(())
}

/// Select a specific entry from the table by UUID
///
/// # Arguments
/// * `table` - Name of the table
/// * `uuid` - Unique identifier of the entry to select
///
/// # Returns
/// Returns the `ComEntry` if found
///
/// # Errors
/// Returns an error if:
/// - The entry does not exist
/// - The database query fails
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let entry = entry::select("users", "user-123").await?;
///     println!("Found entry: {:?}", entry);
///     Ok(())
/// }
/// ```
pub async fn select(table: &str, uuid: &str) -> Result<ComEntry> {
    Ok(
        sqlx::query_as::<_, ComEntry>(&format!("SELECT * FROM {table} WHERE uuid=?"))
            .bind(uuid)
            .fetch_one(&pool().await)
            .await?,
    )
}

/// Select all entries from the table
///
/// # Arguments
/// * `table` - Name of the table
///
/// # Returns
/// Returns a vector of all `ComEntry` records in the table
///
/// # Errors
/// Returns an error if the database query fails
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let entries = entry::select_all("users").await?;
///     println!("Found {} entries", entries.len());
///     Ok(())
/// }
/// ```
pub async fn select_all(table: &str) -> Result<Vec<ComEntry>> {
    Ok(
        sqlx::query_as::<_, ComEntry>(&format!("SELECT * FROM {table}"))
            .fetch_all(&pool().await)
            .await?,
    )
}

/// Get the number of rows in the table
///
/// # Arguments
/// * `table` - Name of the table
///
/// # Returns
/// Returns the count of rows as `i64`
///
/// # Errors
/// Returns an error if the database query fails
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let count = entry::row_counts("users").await?;
///     println!("Table has {} rows", count);
///     Ok(())
/// }
/// ```
pub async fn row_counts(table: &str) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {table}"))
        .fetch_one(&pool().await)
        .await?;

    Ok(count.0)
}

/// Check if an entry exists in the table
///
/// # Arguments
/// * `table` - Name of the table
/// * `uuid` - Unique identifier to check
///
/// # Returns
/// Returns `Ok(())` if the entry exists, otherwise returns an error
///
/// # Errors
/// Returns an error if the entry does not exist
///
/// # Example
/// ```no_run
/// use sqldb::entry;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     if entry::is_exist("users", "user-123").await.is_ok() {
///         println!("Entry exists");
///     } else {
///         println!("Entry does not exist");
///     }
///     Ok(())
/// }
/// ```
pub async fn is_exist(table: &str, uuid: &str) -> Result<()> {
    select(table, uuid).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;

    static MTX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
    const TABLE_NAME: &str = "test";

    /// Initialize test database with test table
    pub async fn init(db_path: &str) {
        super::super::create_db(db_path).await.expect("create db");
        new(TABLE_NAME).await.expect("account table failed");
    }

    /// Test table creation
    #[tokio::test]
    async fn test_table_new() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;

        // Verify table was created by checking if it exists
        super::super::is_table_exist(TABLE_NAME).await?;

        Ok(())
    }

    /// Test deleting all entries from table
    #[tokio::test]
    async fn test_delete_all() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-delete-all.db";

        // Clean up and create fresh database
        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        // Add some data first
        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        insert(TABLE_NAME, "uuid-2", "data-2").await?;

        // Verify data exists
        assert_eq!(row_counts(TABLE_NAME).await?, 2);

        // Delete all
        delete_all(TABLE_NAME).await?;

        // Verify all data is gone
        assert_eq!(row_counts(TABLE_NAME).await?, 0);

        Ok(())
    }

    /// Test deleting specific entry
    #[tokio::test]
    async fn test_delete_one() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-delete-one.db";

        // Clean up and create fresh database
        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        insert(TABLE_NAME, "uuid-2", "data-2").await?;

        // Verify both entries exist
        assert_eq!(row_counts(TABLE_NAME).await?, 2);

        // Delete one entry
        delete(TABLE_NAME, "uuid-1").await?;

        // Verify only one entry remains
        assert_eq!(row_counts(TABLE_NAME).await?, 1);

        // Verify the correct entry was deleted
        assert!(select(TABLE_NAME, "uuid-1").await.is_err());
        assert!(select(TABLE_NAME, "uuid-2").await.is_ok());

        Ok(())
    }

    /// Test inserting entries
    #[tokio::test]
    async fn test_insert() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-insert.db";

        // Clean up and create fresh database
        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        insert(TABLE_NAME, "uuid-2", "data-2").await?;

        // Verify both entries were inserted
        assert_eq!(row_counts(TABLE_NAME).await?, 2);

        // Test unique constraint violation
        assert!(insert(TABLE_NAME, "uuid-1", "duplicate").await.is_err());

        Ok(())
    }

    /// Test updating entries
    #[tokio::test]
    async fn test_update() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;
        new(TABLE_NAME).await?;
        delete_all(TABLE_NAME).await?;

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        update(TABLE_NAME, "uuid-1", "data-1-1").await?;

        assert_eq!(
            select(TABLE_NAME, "uuid-1").await?.data,
            "data-1-1".to_string()
        );

        Ok(())
    }

    /// Test selecting single entry
    #[tokio::test]
    async fn test_select_one() -> Result<()> {
        let _mtx = MTX.lock().await;

        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;
        new(TABLE_NAME).await?;
        delete_all(TABLE_NAME).await?;

        assert!(select(TABLE_NAME, "uuid-1").await.is_err());

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        let item = select(TABLE_NAME, "uuid-1").await?;
        assert_eq!(item.uuid, "uuid-1");
        assert_eq!(item.data, "data-1");
        Ok(())
    }

    /// Test selecting all entries
    #[tokio::test]
    async fn test_select_all() -> Result<()> {
        let _mtx = MTX.lock().await;

        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;
        new(TABLE_NAME).await?;
        delete_all(TABLE_NAME).await?;

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        insert(TABLE_NAME, "uuid-2", "data-2").await?;

        let v = select_all(TABLE_NAME).await?;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].uuid, "uuid-1");
        assert_eq!(v[0].data, "data-1");
        assert_eq!(v[1].uuid, "uuid-2");
        assert_eq!(v[1].data, "data-2");
        Ok(())
    }

    /// Test row counting
    #[tokio::test]
    async fn test_row_counts() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;
        new(TABLE_NAME).await?;
        delete_all(TABLE_NAME).await?;

        assert_eq!(row_counts(TABLE_NAME).await?, 0);
        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        assert_eq!(row_counts(TABLE_NAME).await?, 1);
        insert(TABLE_NAME, "uuid-2", "data-2").await?;
        assert_eq!(row_counts(TABLE_NAME).await?, 2);

        Ok(())
    }

    /// Test entry existence checking
    #[tokio::test]
    async fn test_is_exist() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;
        new(TABLE_NAME).await?;
        delete_all(TABLE_NAME).await?;
        insert(TABLE_NAME, "uuid-1", "data-1").await?;

        assert!(is_exist(TABLE_NAME, "uuid-0").await.is_err());
        assert!(is_exist(TABLE_NAME, "uuid-1").await.is_ok());
        Ok(())
    }

    /// Test comprehensive CRUD operations
    #[tokio::test]
    async fn test_comprehensive_crud() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;
        new(TABLE_NAME).await?;
        delete_all(TABLE_NAME).await?;

        // Create
        insert(TABLE_NAME, "user-1", "user data 1").await?;
        insert(TABLE_NAME, "user-2", "user data 2").await?;

        // Read
        let entry1 = select(TABLE_NAME, "user-1").await?;
        let all_entries = select_all(TABLE_NAME).await?;

        assert_eq!(entry1.data, "user data 1");
        assert_eq!(all_entries.len(), 2);

        // Update
        update(TABLE_NAME, "user-1", "updated user data 1").await?;
        let updated_entry = select(TABLE_NAME, "user-1").await?;
        assert_eq!(updated_entry.data, "updated user data 1");

        // Delete
        delete(TABLE_NAME, "user-2").await?;
        let remaining_entries = select_all(TABLE_NAME).await?;
        assert_eq!(remaining_entries.len(), 1);

        Ok(())
    }

    /// Test error conditions
    #[tokio::test]
    async fn test_error_conditions() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-error-conditions.db";

        // Clean up and create fresh database
        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        // Test selecting non-existent entry
        assert!(select(TABLE_NAME, "non-existent").await.is_err());

        // Test updating non-existent entry
        // Note: SQLite UPDATE on non-existent rows doesn't error, it just affects 0 rows
        update(TABLE_NAME, "non-existent", "data").await?;
        assert_eq!(row_counts(TABLE_NAME).await?, 0);

        // Test deleting non-existent entry
        // Note: SQLite DELETE on non-existent rows doesn't error, it just affects 0 rows
        delete(TABLE_NAME, "non-existent").await?;
        assert_eq!(row_counts(TABLE_NAME).await?, 0);

        Ok(())
    }
}
