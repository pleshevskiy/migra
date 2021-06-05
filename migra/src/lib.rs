#![deny(missing_debug_implementations)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod clients;

pub mod fs;
pub mod managers;
pub mod migration;

mod error;
pub use error::{Error, MigraResult as Result, StdResult};

pub use migration::Migration;
