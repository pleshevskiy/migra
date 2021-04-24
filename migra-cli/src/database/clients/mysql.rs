use crate::database::builder::merge_query_with_params;
use crate::database::prelude::*;
use crate::error::StdResult;
use mysql::prelude::*;
use mysql::{Pool, PooledConn};

pub struct MySqlConnection {
    conn: PooledConn,
}

impl OpenDatabaseConnection for MySqlConnection {
    fn open(connection_string: &str) -> StdResult<Self> {
        let pool = Pool::new_manual(1, 1, connection_string)?;
        let conn = pool.get_conn()?;
        Ok(MySqlConnection { conn })
    }
}

impl DatabaseStatements for MySqlConnection {
    fn create_migration_table_stmt(&self) -> &'static str {
        r#"CREATE TABLE IF NOT EXISTS migrations (
            id      int             AUTO_INCREMENT PRIMARY KEY,
            name    varchar(256)    NOT NULL UNIQUE
        )"#
    }
}

impl SupportsTransactionalDDL for MySqlConnection {}

impl DatabaseConnection for MySqlConnection {
    fn batch_execute(&mut self, query: &str) -> StdResult<()> {
        self.conn.query_drop(query)?;
        Ok(())
    }

    fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64> {
        let stmt = merge_query_with_params(query, params);

        let res = self.conn.query_first(stmt)?.unwrap_or_default();
        Ok(res)
    }

    fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>> {
        let stmt = merge_query_with_params(query, params);

        let res = self.conn.query_map(stmt, |(column,)| vec![column])?;

        Ok(res)
    }
}
