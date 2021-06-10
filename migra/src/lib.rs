#![deny(missing_debug_implementations)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod clients;

mod errors;
pub mod fs;
pub mod managers;
pub mod migration;

pub use clients::{maybe_with_transaction, with_transaction};
pub use errors::{Error, MigraResult as Result, StdResult};
pub use migration::Migration;
