use crate::OpenDatabaseConnection;
use migra::managers::{BatchExecute, ManageMigrations, ManageTransaction};
use migra::migration;
use postgres::{Client, NoTls};
use std::fmt;

pub struct PostgresClient {
    client: Client,
    migrations_table_name: String,
}

impl fmt::Debug for PostgresClient {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("PostgresClient")
            .field("migrations_table_name", &self.migrations_table_name)
            .finish()
    }
}

impl OpenDatabaseConnection for PostgresClient {
    fn manual(connection_string: &str, migrations_table_name: &str) -> migra::Result<Self> {
        let client = Client::connect(connection_string, NoTls)
            .map_err(|_| migra::Error::FailedDatabaseConnection)?;
        Ok(PostgresClient {
            client,
            migrations_table_name: migrations_table_name.to_owned(),
        })
    }
}

impl BatchExecute for PostgresClient {
    fn batch_execute(&mut self, sql: &str) -> migra::StdResult<()> {
        self.client.batch_execute(sql).map_err(From::from)
    }
}

impl ManageTransaction for PostgresClient {}

impl ManageMigrations for PostgresClient {
    fn create_migrations_table(&mut self) -> migra::Result<()> {
        let stmt = format!(
            r#"CREATE TABLE IF NOT EXISTS {} (
                id      serial      PRIMARY KEY,
                name    text        NOT NULL UNIQUE
            )"#,
            &self.migrations_table_name
        );

        self.batch_execute(&stmt)
            .map_err(|_| migra::Error::FailedCreateMigrationsTable)
    }

    fn insert_migration(&mut self, name: &str) -> migra::Result<u64> {
        let stmt = format!(
            "INSERT INTO {} (name) VALUES ($1)",
            &self.migrations_table_name
        );

        self.client
            .execute(stmt.as_str(), &[&name])
            .map_err(|_| migra::Error::FailedInsertMigration)
    }

    fn delete_migration(&mut self, name: &str) -> migra::Result<u64> {
        let stmt = format!(
            "DELETE FROM {} WHERE name = $1",
            &self.migrations_table_name
        );

        self.client
            .execute(stmt.as_str(), &[&name])
            .map_err(|_| migra::Error::FailedDeleteMigration)
    }

    fn applied_migrations(&mut self) -> migra::Result<migration::List> {
        let stmt = format!("SELECT name FROM {}", &self.migrations_table_name);

        self.client
            .query(stmt.as_str(), &[])
            .and_then(|res| {
                res.into_iter()
                    .map(|row| row.try_get(0))
                    .collect::<Result<Vec<String>, _>>()
            })
            .map(From::from)
            .map_err(|_| migra::Error::FailedGetAppliedMigrations)
    }
}
