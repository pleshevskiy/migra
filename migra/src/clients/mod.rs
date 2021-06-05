// #![deny(missing_debug_implementations)]
// #![deny(clippy::all, clippy::pedantic)]
// #![allow(clippy::module_name_repetitions)]
// #![allow(clippy::missing_errors_doc)]

use crate::error::MigraResult;
use crate::managers::{ManageMigrations, ManageTransaction};

pub trait OpenDatabaseConnection
where
    Self: Sized,
{
    fn new(connection_string: &str) -> MigraResult<Self> {
        Self::manual(connection_string, "migrations")
    }

    fn manual(connection_string: &str, migrations_table_name: &str) -> MigraResult<Self>;
}

pub trait Client: ManageMigrations + ManageTransaction {}

pub type AnyClient = Box<dyn Client>;

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "postgres")]
pub use self::postgres::Client as PostgresClient;

#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::Client as MysqlClient;

#[cfg(feature = "sqlite")]
pub mod sqlite;
#[cfg(feature = "sqlite")]
pub use self::sqlite::Client as SqliteClient;
