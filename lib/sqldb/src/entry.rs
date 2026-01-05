use super::{ComEntry, pool};
use anyhow::Result;

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

pub async fn delete(table: &str, uuid: &str) -> Result<()> {
    sqlx::query(&format!("DELETE FROM {table} WHERE uuid=?"))
        .bind(uuid)
        .execute(&pool().await)
        .await?;
    Ok(())
}

pub async fn delete_all(table: &str) -> Result<()> {
    sqlx::query(&format!("DELETE FROM {table}"))
        .execute(&pool().await)
        .await?;
    Ok(())
}

pub async fn insert(table: &str, uuid: &str, data: &str) -> Result<()> {
    sqlx::query(&format!("INSERT INTO {table} (uuid, data) VALUES (?, ?)"))
        .bind(uuid)
        .bind(data)
        .execute(&pool().await)
        .await?;
    Ok(())
}

pub async fn update(table: &str, uuid: &str, data: &str) -> Result<()> {
    sqlx::query(&format!("UPDATE {table} SET data=? WHERE uuid=?"))
        .bind(data)
        .bind(uuid)
        .execute(&pool().await)
        .await?;

    Ok(())
}

pub async fn select(table: &str, uuid: &str) -> Result<ComEntry> {
    Ok(
        sqlx::query_as::<_, ComEntry>(&format!("SELECT * FROM {table} WHERE uuid=?"))
            .bind(uuid)
            .fetch_one(&pool().await)
            .await?,
    )
}

pub async fn select_all(table: &str) -> Result<Vec<ComEntry>> {
    Ok(
        sqlx::query_as::<_, ComEntry>(&format!("SELECT * FROM {table}"))
            .fetch_all(&pool().await)
            .await?,
    )
}

pub async fn row_counts(table: &str) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {table}"))
        .fetch_one(&pool().await)
        .await?;

    Ok(count.0)
}

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

    pub async fn init(db_path: &str) {
        super::super::create_db(db_path).await.expect("create db");
        new(TABLE_NAME).await.expect("account table failed");
    }

    #[tokio::test]
    async fn test_table_new() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;

        super::super::is_table_exist(TABLE_NAME).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_all() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-delete-all.db";

        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        insert(TABLE_NAME, "uuid-2", "data-2").await?;

        assert_eq!(row_counts(TABLE_NAME).await?, 2);

        delete_all(TABLE_NAME).await?;

        assert_eq!(row_counts(TABLE_NAME).await?, 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_one() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-delete-one.db";

        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        insert(TABLE_NAME, "uuid-2", "data-2").await?;

        assert_eq!(row_counts(TABLE_NAME).await?, 2);

        delete(TABLE_NAME, "uuid-1").await?;

        assert_eq!(row_counts(TABLE_NAME).await?, 1);

        assert!(select(TABLE_NAME, "uuid-1").await.is_err());
        assert!(select(TABLE_NAME, "uuid-2").await.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_insert() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-insert.db";

        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        insert(TABLE_NAME, "uuid-1", "data-1").await?;
        insert(TABLE_NAME, "uuid-2", "data-2").await?;

        assert_eq!(row_counts(TABLE_NAME).await?, 2);

        assert!(insert(TABLE_NAME, "uuid-1", "duplicate").await.is_err());

        Ok(())
    }

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

    #[tokio::test]
    async fn test_comprehensive_crud() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-table-new.db";
        init(test_db_path).await;
        new(TABLE_NAME).await?;
        delete_all(TABLE_NAME).await?;

        insert(TABLE_NAME, "user-1", "user data 1").await?;
        insert(TABLE_NAME, "user-2", "user data 2").await?;

        let entry1 = select(TABLE_NAME, "user-1").await?;
        let all_entries = select_all(TABLE_NAME).await?;

        assert_eq!(entry1.data, "user data 1");
        assert_eq!(all_entries.len(), 2);

        update(TABLE_NAME, "user-1", "updated user data 1").await?;
        let updated_entry = select(TABLE_NAME, "user-1").await?;
        assert_eq!(updated_entry.data, "updated user data 1");

        delete(TABLE_NAME, "user-2").await?;
        let remaining_entries = select_all(TABLE_NAME).await?;
        assert_eq!(remaining_entries.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_error_conditions() -> Result<()> {
        let _mtx = MTX.lock().await;
        let test_db_path = "/tmp/test-error-conditions.db";

        let _ = std::fs::remove_file(test_db_path);
        super::super::create_db(test_db_path).await?;
        new(TABLE_NAME).await?;

        assert!(select(TABLE_NAME, "non-existent").await.is_err());

        update(TABLE_NAME, "non-existent", "data").await?;
        assert_eq!(row_counts(TABLE_NAME).await?, 0);

        delete(TABLE_NAME, "non-existent").await?;
        assert_eq!(row_counts(TABLE_NAME).await?, 0);

        Ok(())
    }
}
