use super::OpenDatabaseConnection;
use crate::error::{Error, MigraResult, StdResult};
use crate::managers::{BatchExecute, ManageMigrations, ManageTransaction};
use crate::migration;
use rusqlite::Connection;

#[derive(Debug)]
pub struct Client {
    conn: Connection,
    migrations_table_name: String,
}

impl OpenDatabaseConnection for Client {
    fn manual(connection_string: &str, migrations_table_name: &str) -> MigraResult<Self> {
        let conn =
            Connection::open(connection_string).map_err(|_| Error::FailedDatabaseConnection)?;
        Ok(Client {
            conn,
            migrations_table_name: migrations_table_name.to_owned(),
        })
    }
}

impl BatchExecute for Client {
    fn batch_execute(&mut self, sql: &str) -> StdResult<()> {
        self.conn.execute_batch(sql).map_err(From::from)
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
            .map_err(|_| Error::FailedCreateMigrationsTable)
    }

    fn insert_migration(&mut self, name: &str) -> MigraResult<u64> {
        let stmt = format!(
            "INSERT INTO {} (name) VALUES ($1)",
            &self.migrations_table_name
        );

        self.conn
            .execute(&stmt, [name])
            .map(|res| res as u64)
            .map_err(|_| Error::FailedInsertMigration)
    }

    fn delete_migration(&mut self, name: &str) -> MigraResult<u64> {
        let stmt = format!(
            "DELETE FROM {} WHERE name = $1",
            &self.migrations_table_name
        );

        self.conn
            .execute(&stmt, [name])
            .map(|res| res as u64)
            .map_err(|_| Error::FailedDeleteMigration)
    }

    fn applied_migrations(&mut self) -> MigraResult<migration::List> {
        let stmt = format!("SELECT name FROM {}", &self.migrations_table_name);

        self.conn
            .prepare(&stmt)
            .and_then(|mut stmt| {
                stmt.query_map([], |row| row.get(0))?
                    .collect::<Result<Vec<String>, _>>()
            })
            .map(From::from)
            .map_err(|_| Error::FailedGetAppliedMigrations)
    }
}

impl super::Client for Client {}
