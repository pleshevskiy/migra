pub(crate) mod adapter;
pub(crate) mod builder;
pub(crate) mod clients;
pub(crate) mod connection;
pub(crate) mod migration;
pub(crate) mod transaction;

pub mod prelude {
    pub use super::adapter::{ToSql, ToSqlParams, TryFromSql};
    pub use super::connection::{
        AnyConnection, DatabaseConnection, DatabaseStatements, OpenDatabaseConnection,
        SupportsTransactionalDdl,
    };
    pub use super::migration::ManageMigration;
    pub use super::transaction::ManageTransaction;
}

pub(crate) use connection::DatabaseConnectionManager;
pub(crate) use migration::{Migration, MigrationManager};
