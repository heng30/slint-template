// use serde::{Deserialize, Deserializer, Serialize, Serializer};
// use serde_with::{serde_as, DeserializeAs, SerializeAs};

// use crate::slint_generatedAppWindow::{
//     AccountEntry as UIAccountEntry, AddressBookEntry as UIAddressBookEntry,
//     TokenTileEntry as UITokenTileEntry, TransactionTileEntry as UIHistoryEntry,
//     TransactionTileStatus,
// };
// pub const ACCOUNTS_TABLE: &str = "accounts";
// pub const ADDRESS_BOOK_TABLE: &str = "address_book";
// pub const HISTORY_TABLE: &str = "history";
// pub const TOKENS_TABLE: &str = "tokens";

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
