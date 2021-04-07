use crate::database::builder::merge_query_with_params;
use crate::database::prelude::*;
use crate::error::StdResult;
use mysql::prelude::*;
use mysql::{Pool, PooledConn};

pub struct MySqlConnection {
    pool: Pool,
}

impl MySqlConnection {
    fn client(&self) -> StdResult<PooledConn> {
        let conn = self.pool.get_conn()?;
        Ok(conn)
    }
}

impl OpenDatabaseConnection for MySqlConnection {
    fn open(connection_string: &str) -> StdResult<Self> {
        let pool = Pool::new(connection_string)?;
        Ok(MySqlConnection { pool })
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

impl DatabaseConnection for MySqlConnection {
    fn batch_execute(&mut self, query: &str) -> StdResult<()> {
        self.client()?.query_drop(query)?;
        Ok(())
    }

    fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64> {
        let stmt = merge_query_with_params(query, params);

        let res = self.client()?.query_first(stmt)?.unwrap_or_default();
        Ok(res)
    }

    fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>> {
        let stmt = merge_query_with_params(query, params);

        let res = self.client()?.query_map(stmt, |(column,)| vec![column])?;

        Ok(res)
    }
}
