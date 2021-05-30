use crate::error::MigraResult;
use migra::managers::{ManageMigrations, ManageTransaction};
use migra::migration;
use mysql::prelude::*;
use mysql::{Pool, PooledConn};

pub struct MySqlClient {
    conn: PooledConn,
    migrations_table_name: String,
}

impl MySqlClient {
    fn new(connection_string: &str, migrations_table_name: &str) -> MigraResult<Self> {
        let conn = Pool::new_manual(1, 1, connection_string)
            .and_then(|pool| pool.get_conn())
            .map_err(|_| crate::Error::FailedDatabaseConnection)?;

        Ok(MySqlClient {
            conn,
            migrations_table_name: migrations_table_name.to_owned(),
        })
    }
}

impl ManageTransaction for MySqlClient {
    fn begin_transaction(&mut self) -> migra::Result<()> {
        self.conn
            .query_drop("BEGIN")
            .map_err(|_| migra::Error::FailedOpenTransaction)
    }

    fn rollback_transaction(&mut self) -> migra::Result<()> {
        self.conn
            .query_drop("ROLLBACK")
            .map_err(|_| migra::Error::FailedRollbackTransaction)
    }

    fn commit_transaction(&mut self) -> migra::Result<()> {
        self.conn
            .query_drop("COMMIT")
            .map_err(|_| migra::Error::FailedCommitTransaction)
    }
}

impl ManageMigrations for MySqlClient {
    fn apply_sql(&mut self, sql_content: &str) -> migra::Result<()> {
        self.conn
            .query_drop(sql_content)
            .map_err(|_| migra::Error::FailedApplySql)
    }

    fn create_migrations_table(&mut self) -> migra::Result<()> {
        let stmt = format!(
            r#"CREATE TABLE IF NOT EXISTS {} (
                id      int             AUTO_INCREMENT PRIMARY KEY,
                name    varchar(256)    NOT NULL UNIQUE
            )"#,
            &self.migrations_table_name
        );

        self.conn
            .query_drop(stmt)
            .map_err(|_| migra::Error::FailedCreateMigrationsTable)
    }

    fn insert_migration(&mut self, name: &str) -> migra::Result<u64> {
        let stmt = format!(
            "INSERT INTO {} (name) VALUES ($1)",
            &self.migrations_table_name
        );

        self.conn
            .exec_first(&stmt, (name,))
            .map(|res| res.unwrap_or_default())
            .map_err(|_| migra::Error::FailedInsertMigration)
    }

    fn delete_migration(&mut self, name: &str) -> migra::Result<u64> {
        let stmt = format!(
            "DELETE FROM {} WHERE name = $1",
            &self.migrations_table_name
        );

        self.conn
            .exec_first(&stmt, (name,))
            .map(|res| res.unwrap_or_default())
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
