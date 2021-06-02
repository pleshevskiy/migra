use crate::OpenDatabaseConnection;
use migra::managers::{BatchExecute, ManageMigrations, ManageTransaction};
use migra::migration;
use rusqlite::Connection;

#[derive(Debug)]
pub struct SqliteClient {
    conn: Connection,
    migrations_table_name: String,
}

impl OpenDatabaseConnection for SqliteClient {
    fn manual(connection_string: &str, migrations_table_name: &str) -> migra::Result<Self> {
        let conn = Connection::open(connection_string)
            .map_err(|_| migra::Error::FailedDatabaseConnection)?;
        Ok(SqliteClient {
            conn,
            migrations_table_name: migrations_table_name.to_owned(),
        })
    }
}

impl BatchExecute for SqliteClient {
    fn batch_execute(&mut self, sql: &str) -> migra::StdResult<()> {
        self.conn.execute_batch(sql).map_err(From::from)
    }
}

impl ManageTransaction for SqliteClient {}

impl ManageMigrations for SqliteClient {
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
            .execute(&stmt, [name])
            .map(|res| res as u64)
            .map_err(|_| migra::Error::FailedInsertMigration)
    }

    fn delete_migration(&mut self, name: &str) -> migra::Result<u64> {
        let stmt = format!(
            "DELETE FROM {} WHERE name = $1",
            &self.migrations_table_name
        );

        self.conn
            .execute(&stmt, [name])
            .map(|res| res as u64)
            .map_err(|_| migra::Error::FailedDeleteMigration)
    }

    fn applied_migrations(&mut self) -> migra::Result<migration::List> {
        let stmt = format!("SELECT name FROM {}", &self.migrations_table_name);

        self.conn
            .prepare(&stmt)
            .and_then(|mut stmt| {
                stmt.query_map([], |row| row.get(0))?
                    .collect::<Result<Vec<String>, _>>()
            })
            .map(From::from)
            .map_err(|_| migra::Error::FailedGetAppliedMigrations)
    }
}

// impl DatabaseConnection for SqliteConnection {
//     fn batch_execute(&mut self, query: &str) -> StdResult<()> {
//         self.conn.execute_batch(query)?;
//         Ok(())
//     }

//     fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64> {
//         let stmt = merge_query_with_params(query, params);

//         let res = self.conn.execute(&stmt, [])?;
//         Ok(res as u64)
//     }

//     fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>> {
//         let stmt = merge_query_with_params(query, params);

//         let mut stmt = self.conn.prepare(&stmt)?;

//         let res = stmt
//             .query_map([], |row| Ok(vec![row.get(0)?]))?
//             .collect::<Result<_, _>>()?;

//         Ok(res)
//     }
// }
