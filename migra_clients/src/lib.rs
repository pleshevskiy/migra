#![deny(missing_debug_implementations)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]

trait OpenDatabaseConnection: Sized {
    fn new(connection_string: &str) -> migra::Result<Self> {
        Self::manual(connection_string, "migrations")
    }

    fn manual(connection_string: &str, migrations_table_name: &str) -> migra::Result<Self>;
}

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "postgres")]
pub use self::postgres::*;

#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::*;

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "sqlite")]
pub use self::sqlite::*;
