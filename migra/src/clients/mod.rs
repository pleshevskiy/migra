use crate::errors::MigraResult;
use crate::managers::{ManageMigrations, ManageTransaction};

/// A trait that helps to open a connection to a specific database client.
pub trait OpenDatabaseConnection
where
    Self: Sized,
{
    /// Open database connection with predefined migrations table name.
    fn new(connection_string: &str) -> MigraResult<Self> {
        Self::manual(connection_string, "migrations")
    }

    /// Open database connection manually with additional migration table name parameter.
    fn manual(connection_string: &str, migrations_table_name: &str) -> MigraResult<Self>;
}

/// All client implementations that have migration and transaction manager implementations
/// are considered clients.
pub trait Client: ManageMigrations + ManageTransaction {}

/// If you have complex application mechanics that allow users to choose which
/// database they can use, then you will most likely need this helper for that.
pub type AnyClient = Box<(dyn Client + 'static)>;

#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "postgres")]
pub use self::postgres::Client as PostgresClient;

#[cfg(feature = "mysql")]
mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::Client as MysqlClient;

#[cfg(feature = "sqlite")]
mod sqlite;
#[cfg(feature = "sqlite")]
pub use self::sqlite::Client as SqliteClient;
