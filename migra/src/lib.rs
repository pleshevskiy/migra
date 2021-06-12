//! # Migra
//!
//! Migra is a simple library for managing SQL in your application.
//!
//! For example, if you have a task list application, you can update the local user database from version to version.
//!
//! This is main crate for [migra-cli](https://crates.io/crates/migra-cli), which allows you to manege SQL for web
//! servers in any program language without being bound to SQL frameworks.
//!
//! ## Installation
//!
//! Add `migra = { version = "1.0" }` as a dependency in `Cargo.toml`.
//!
//! This crate has not required predefined database clients in features with similar name.
//! If you want to add them, just install crate with additional features (`postgres`, `mysql`, `sqlite`).
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [package]
//! name = "my-crate"
//! version = "0.1.0"
//! authors = ["Me <user@rust-lang.org>"]
//!
//! [dependencies]
//! migra = { version = "1.0", features = ["postgres"] }
//! ```
//!
//! ### Supported databases
//!
//! | Database Client | Feature      |
//! |-----------------|--------------|
//! | `Postgres`      | postgres     |
//! | `MySQL`         | mysql        |
//! | `Sqlite`        | sqlite       |
//!
#![deny(missing_debug_implementations)]
#![warn(missing_docs)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

/// Includes additional client tools and contains predefined
/// database clients that have been enabled in the features.
pub mod clients;

/// Includes all types of errors that uses in the crate.
pub mod errors;

/// Includes utilities that use the file system to work.
pub mod fs;

/// Includes all the basic traits that will allow you
/// to create your own client.
pub mod managers;

/// Includes basic structures of migration and migration
/// lists, that are used in managers and fs utils.
pub mod migration;

pub use errors::{Error, MigraResult as Result, StdResult};
pub use migration::{List as MigrationList, Migration};
