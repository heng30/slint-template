//! # CUtil - Common Utilities Library
//!
//! A collection of utility modules providing common functionality for file system operations,
//! string manipulation, time handling, HTTP requests, cryptography, and more.
//!
//! ## Features
//!
//! - `fs`: File system utilities (file operations, directory management, size calculations)
//! - `str`: String manipulation utilities (splitting, formatting, random generation)
//! - `time`: Time and date utilities (formatting, parsing, calendar operations)
//! - `http`: HTTP client utilities (requests, headers, URL parsing)
//! - `crypto`: Cryptographic utilities (encryption, decryption, hashing)
//! - `number`: Number formatting utilities
//! - `backup-recover`: Backup and restore utilities
//! - `vec`: Vector manipulation utilities

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "str")]
pub mod str;

#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "crypto")]
pub mod crypto;

#[cfg(feature = "number")]
pub mod number;

#[cfg(feature = "backup-recover")]
pub mod backup_recover;

#[cfg(feature = "vec")]
pub mod vec;
