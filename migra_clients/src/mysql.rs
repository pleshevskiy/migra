use crate::OpenDatabaseConnection;
use migra::managers::{BatchExecute, ManageMigrations, ManageTransaction};
use migra::migration;
use mysql::prelude::*;
use mysql::{Pool, PooledConn};

#[derive(Debug)]
pub struct MySqlClient {
    conn: PooledConn,
    migrations_table_name: String,
}

impl OpenDatabaseConnection for MySqlClient {
    fn manual(connection_string: &str, migrations_table_name: &str) -> migra::Result<Self> {
        let conn = Pool::new_manual(1, 1, connection_string)
            .and_then(|pool| pool.get_conn())
            .map_err(|_| migra::Error::FailedDatabaseConnection)?;

        Ok(MySqlClient {
            conn,
            migrations_table_name: migrations_table_name.to_owned(),
        })
    }
}

impl BatchExecute for MySqlClient {
    fn batch_execute(&mut self, sql: &str) -> migra::StdResult<()> {
        self.conn.query_drop(sql).map_err(From::from)
    }
}

impl ManageTransaction for MySqlClient {}

impl ManageMigrations for MySqlClient {
    fn create_migrations_table(&mut self) -> migra::Result<()> {
        let stmt = format!(
            r#"CREATE TABLE IF NOT EXISTS {} (
                id      int             AUTO_INCREMENT PRIMARY KEY,
                name    varchar(256)    NOT NULL UNIQUE
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

        self.conn
            .exec_first(&stmt, (name,))
            .map(Option::unwrap_or_default)
            .map_err(|_| migra::Error::FailedInsertMigration)
    }

    fn delete_migration(&mut self, name: &str) -> migra::Result<u64> {
        let stmt = format!(
            "DELETE FROM {} WHERE name = $1",
            &self.migrations_table_name
        );

        self.conn
            .exec_first(&stmt, (name,))
            .map(Option::unwrap_or_default)
            .map_err(|_| migra::Error::FailedDeleteMigration)
    }

    fn applied_migrations(&mut self) -> migra::Result<migration::List> {
        let stmt = format!("SELECT name FROM {}", &self.migrations_table_name);

        self.conn
            .query::<String, _>(stmt)
            .map(From::from)
            .map_err(|_| migra::Error::FailedGetAppliedMigrations)
    }
}
