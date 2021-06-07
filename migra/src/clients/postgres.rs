use super::OpenDatabaseConnection;
use crate::errors::{DbKind, Error, MigraResult, StdResult};
use crate::managers::{BatchExecute, ManageMigrations, ManageTransaction};
use crate::migration;
use postgres::{Client as PostgresClient, NoTls};
use std::fmt;

pub struct Client {
    conn: PostgresClient,
    migrations_table_name: String,
}

impl Client {
    #[must_use]
    pub fn conn(&self) -> &PostgresClient {
        &self.conn
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Client")
            .field("migrations_table_name", &self.migrations_table_name)
            .finish()
    }
}

impl OpenDatabaseConnection for Client {
    fn manual(connection_string: &str, migrations_table_name: &str) -> MigraResult<Self> {
        let conn = PostgresClient::connect(connection_string, NoTls)
            .map_err(|err| Error::db(err.into(), DbKind::DatabaseConnection))?;
        Ok(Client {
            conn,
            migrations_table_name: migrations_table_name.to_owned(),
        })
    }
}

impl BatchExecute for Client {
    fn batch_execute(&mut self, sql: &str) -> StdResult<()> {
        self.conn.batch_execute(sql).map_err(From::from)
    }
}

impl ManageTransaction for Client {}

impl ManageMigrations for Client {
    fn create_migrations_table(&mut self) -> MigraResult<()> {
        let stmt = format!(
            r#"CREATE TABLE IF NOT EXISTS {} (
                id      serial      PRIMARY KEY,
                name    text        NOT NULL UNIQUE
            )"#,
            &self.migrations_table_name
        );

        self.batch_execute(&stmt)
            .map_err(|err| Error::db(err, DbKind::CreateMigrationsTable))
    }

    fn insert_migration(&mut self, name: &str) -> MigraResult<u64> {
        let stmt = format!(
            "INSERT INTO {} (name) VALUES ($1)",
            &self.migrations_table_name
        );

        self.conn
            .execute(stmt.as_str(), &[&name])
            .map_err(|err| Error::db(err.into(), DbKind::InsertMigration))
    }

    fn delete_migration(&mut self, name: &str) -> MigraResult<u64> {
        let stmt = format!(
            "DELETE FROM {} WHERE name = $1",
            &self.migrations_table_name
        );

        self.conn
            .execute(stmt.as_str(), &[&name])
            .map_err(|err| Error::db(err.into(), DbKind::DeleteMigration))
    }

    fn get_applied_migrations(&mut self) -> MigraResult<migration::List> {
        let stmt = format!(
            "SELECT name FROM {} ORDER BY id DESC",
            &self.migrations_table_name
        );

        self.conn
            .query(stmt.as_str(), &[])
            .and_then(|res| {
                res.into_iter()
                    .map(|row| row.try_get(0))
                    .collect::<Result<Vec<String>, _>>()
            })
            .map(From::from)
            .map_err(|err| Error::db(err.into(), DbKind::GetAppliedMigrations))
    }
}

impl super::Client for Client {}
