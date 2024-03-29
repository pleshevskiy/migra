use super::OpenDatabaseConnection;
use crate::errors::{DbKind, Error, MigraResult, StdResult};
use crate::managers::{BatchExecute, ManageMigrations, ManageTransaction};
use crate::migration;
use mysql::prelude::*;
use mysql::{Pool, PooledConn};

/// Predefined `MySQL` client.
///
/// **Note:** Requires enabling `mysql` feature.
#[derive(Debug)]
pub struct Client {
    conn: PooledConn,
    migrations_table_name: String,
}

impl Client {
    /// Provide access to the original database connection.
    #[must_use]
    pub fn conn(&self) -> &PooledConn {
        &self.conn
    }
}

impl OpenDatabaseConnection for Client {
    fn manual(connection_string: &str, migrations_table_name: &str) -> MigraResult<Self> {
        let conn = Pool::new_manual(1, 1, connection_string)
            .and_then(|pool| pool.get_conn())
            .map_err(|err| Error::db(err.into(), DbKind::DatabaseConnection))?;

        Ok(Client {
            conn,
            migrations_table_name: migrations_table_name.to_owned(),
        })
    }
}

impl BatchExecute for Client {
    fn batch_execute(&mut self, sql: &str) -> StdResult<()> {
        self.conn.query_drop(sql).map_err(From::from)
    }
}

impl ManageTransaction for Client {}

impl ManageMigrations for Client {
    fn create_migrations_table(&mut self) -> MigraResult<()> {
        let stmt = format!(
            r#"CREATE TABLE IF NOT EXISTS {} (
                id      int             AUTO_INCREMENT PRIMARY KEY,
                name    varchar(256)    NOT NULL UNIQUE
            )"#,
            &self.migrations_table_name
        );

        self.batch_execute(&stmt)
            .map_err(|err| Error::db(err, DbKind::CreateMigrationsTable))
    }

    fn insert_migration(&mut self, name: &str) -> MigraResult<u64> {
        let stmt = format!(
            "INSERT INTO {} (name) VALUES (?)",
            &self.migrations_table_name
        );

        self.conn
            .exec_first(&stmt, (name,))
            .map(Option::unwrap_or_default)
            .map_err(|err| Error::db(err.into(), DbKind::InsertMigration))
    }

    fn delete_migration(&mut self, name: &str) -> MigraResult<u64> {
        let stmt = format!("DELETE FROM {} WHERE name = ?", &self.migrations_table_name);

        self.conn
            .exec_first(&stmt, (name,))
            .map(Option::unwrap_or_default)
            .map_err(|err| Error::db(err.into(), DbKind::DeleteMigration))
    }

    fn get_applied_migrations(&mut self) -> MigraResult<migration::List> {
        let stmt = format!(
            "SELECT name FROM {} ORDER BY id DESC",
            &self.migrations_table_name
        );

        self.conn
            .query::<String, _>(stmt)
            .map(From::from)
            .map_err(|err| Error::db(err.into(), DbKind::GetAppliedMigrations))
    }
}

impl super::Client for Client {}
