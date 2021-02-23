pub(crate) mod adapter;
pub(crate) mod builder;
pub(crate) mod clients;
pub(crate) mod connection;
pub(crate) mod migration;

pub mod prelude {
    pub use super::adapter::{ToSql, ToSqlParams, TryFromSql};
    pub use super::connection::{DatabaseConnection, OpenDatabaseConnection};
    pub use super::migration::DatabaseMigrationManager;
}

pub(crate) use connection::DatabaseConnectionManager;
pub(crate) use migration::{Migration, MigrationManager};
