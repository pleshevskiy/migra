use crate::database::builder::merge_query_with_params;
use crate::database::prelude::*;
use crate::error::StdResult;
use rusqlite::Connection;

pub struct SqliteConnection {
    conn: Connection,
}

impl OpenDatabaseConnection for SqliteConnection {
    fn open(connection_string: &str) -> StdResult<Self> {
        let conn = Connection::open(connection_string)?;
        Ok(SqliteConnection { conn })
    }
}

impl DatabaseStatements for SqliteConnection {
    fn create_migration_table_stmt(&self, migrations_table_name: &str) -> String {
        format!(
            r#"CREATE TABLE IF NOT EXISTS {} (
                id      int             AUTO_INCREMENT PRIMARY KEY,
                name    varchar(256)    NOT NULL UNIQUE
            )"#,
            migrations_table_name
        )
    }
}

impl SupportsTransactionalDdl for SqliteConnection {
    #[inline]
    fn supports_transactional_ddl(&self) -> bool {
        true
    }
}

impl DatabaseConnection for SqliteConnection {
    fn batch_execute(&mut self, query: &str) -> StdResult<()> {
        self.conn.execute_batch(query)?;
        Ok(())
    }

    fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64> {
        let stmt = merge_query_with_params(query, params);

        let res = self.conn.execute(&stmt, [])?;
        Ok(res as u64)
    }

    fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>> {
        let stmt = merge_query_with_params(query, params);

        let mut stmt = self.conn.prepare(&stmt)?;

        let res = stmt
            .query_map([], |row| Ok(vec![row.get(0)?]))?
            .collect::<Result<_, _>>()?;

        Ok(res)
    }
}
