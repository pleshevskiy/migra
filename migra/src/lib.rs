#![deny(missing_debug_implementations)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod clients;

mod errors;
pub mod fs;
pub mod managers;
pub mod migration;

pub use clients::{run_in_transaction, should_run_in_transaction};
pub use errors::{Error, MigraResult as Result, StdResult};
pub use migration::Migration;
