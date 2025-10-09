//! Database management module
//! 
//! Provides database initialization and CRUD operation macros for SQLite database.
//! Supports asynchronous database operations with error handling.

// use serde::{Deserialize, Deserializer, Serialize, Serializer};
// use serde_with::{serde_as, DeserializeAs, SerializeAs};

// use crate::slint_generatedAppWindow::{
//     AccountEntry as UIAccountEntry, AddressBookEntry as UIAddressBookEntry,
//     TokenTileEntry as UITokenTileEntry, TransactionTileEntry as UIHistoryEntry,
//     TransactionTileStatus,
// };
// pub const ACCOUNTS_TABLE: &str = "accounts";

/// Initializes the database
/// 
/// Creates the SQLite database file and sets up required tables.
/// 
/// # Parameters
/// - `db_path`: Path to the database file
/// 
/// # Panics
/// - If database creation fails
pub async fn init(db_path: &str) {
    sqldb::create_db(db_path).await.expect("create db");

    // sqldb::entry::new(def::ACCOUNTS_TABLE)
    //     .await
    //     .expect("account table failed");
}

/// Macro for adding entries to the database
/// 
/// Creates an async function that serializes the entry and inserts it into the database.
/// Automatically handles error reporting through toast notifications.
/// 
/// # Parameters
/// - `$table`: Database table name
/// - `$ty`: Entry type that implements `Serialize`
#[macro_export]
macro_rules! db_add {
    ($table:expr, $ty:ident) => {
        fn db_add(ui: slint::Weak<crate::slint_generatedAppWindow::AppWindow>, entry: $ty) {
            tokio::spawn(async move {
                let data = serde_json::to_string(&entry).expect("Not implement `Serialize` trait");
                if let Err(e) = crate::db::entry::insert($table, entry.id.as_str(), &data).await {
                    crate::logic::toast::async_toast_warn(
                        ui,
                        format!("{}. {e}", crate::logic::tr::tr("insert entry failed")),
                    );
                }
            });
        }
    };
}

/// Macro for updating entries in the database
/// 
/// Creates an async function that serializes the entry and updates it in the database.
/// Automatically handles error reporting through toast notifications.
/// 
/// # Parameters
/// - `$table`: Database table name
/// - `$ty`: Entry type that implements `Serialize`
#[macro_export]
macro_rules! db_update {
    ($table:expr, $ty:ident) => {
        fn db_update(ui: slint::Weak<crate::slint_generatedAppWindow::AppWindow>, entry: $ty) {
            tokio::spawn(async move {
                let data = serde_json::to_string(&entry).expect("Not implement `Serialize` trait");
                if let Err(e) = crate::db::entry::update($table, entry.id.as_str(), &data).await {
                    crate::logic::toast::async_toast_warn(
                        ui,
                        format!("{}. {e}", crate::logic::tr::tr("update entry failed")),
                    );
                }
            });
        }
    };
}

/// Macro for removing entries from the database
/// 
/// Creates an async function that deletes an entry from the database by ID.
/// Automatically handles error reporting through toast notifications.
/// 
/// # Parameters
/// - `$table`: Database table name
#[macro_export]
macro_rules! db_remove {
    ($table:expr) => {
        fn db_remove(
            ui: slint::Weak<crate::slint_generatedAppWindow::AppWindow>,
            id: impl ToString,
        ) {
            let id = id.to_string();
            tokio::spawn(async move {
                if let Err(e) = crate::db::entry::delete($table, id.as_str()).await {
                    crate::logic::toast::async_toast_warn(
                        ui,
                        format!("{}. {e}", crate::logic::tr::tr("remove entry failed")),
                    );
                }
            });
        }
    };
}

// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
// pub struct AccountEntry {
//     pub uuid: String,
//     pub name: String,
//     pub pubkey: String,
//     pub derive_index: i32,
//     pub avatar_index: i32,
// }

// impl From<UIAccountEntry> for AccountEntry {
//     fn from(entry: UIAccountEntry) -> Self {
//         AccountEntry {
//             uuid: entry.uuid.into(),
//             name: entry.name.into(),
//             pubkey: entry.pubkey.into(),
//             derive_index: entry.derive_index,
//             avatar_index: entry.avatar_index,
//         }
//     }
// }

// impl From<AccountEntry> for UIAccountEntry {
//     fn from(entry: AccountEntry) -> Self {
//         UIAccountEntry {
//             uuid: entry.uuid.into(),
//             name: entry.name.into(),
//             pubkey: entry.pubkey.into(),
//             derive_index: entry.derive_index,
//             avatar_index: entry.avatar_index,
//         }
//     }
// }

// #[serde_as]
// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
// pub struct HistoryEntry {
//     pub uuid: String,
//     pub network: String,
//     pub hash: String,
//     pub balance: String,
//     pub time: String,

//     #[serde_as(as = "TranStatus")]
//     pub status: TransactionTileStatus,
// }

// struct TranStatus;
// impl SerializeAs<TransactionTileStatus> for TranStatus {
//     fn serialize_as<S>(source: &TransactionTileStatus, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let status = match source {
//             TransactionTileStatus::Success => "Success",
//             TransactionTileStatus::Pending => "Pending",
//             _ => "Error",
//         };

//         serializer.serialize_str(status)
//     }
// }

// impl<'de> DeserializeAs<'de, TransactionTileStatus> for TranStatus {
//     fn deserialize_as<D>(deserializer: D) -> Result<TransactionTileStatus, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let status = String::deserialize(deserializer)?;
//         let status = match status.as_str() {
//             "Success" => TransactionTileStatus::Success,
//             "Pending" => TransactionTileStatus::Pending,
//             _ => TransactionTileStatus::Error,
//         };
//         Ok(status)
//     }
// }
