use crate::database::builder::merge_query_with_params;
use crate::database::prelude::*;
use crate::error::StdResult;
use postgres::{Client, NoTls};

pub struct PostgresConnection {
    client: Client,
}

impl OpenDatabaseConnection for PostgresConnection {
    fn open(connection_string: &str) -> StdResult<Self> {
        let client = Client::connect(connection_string, NoTls)?;
        Ok(PostgresConnection { client })
    }
}

impl DatabaseStatements for PostgresConnection {
    fn create_migration_table_stmt(&self, migrations_table_name: &str) -> String {
        format!(
            r#"CREATE TABLE IF NOT EXISTS {} (
                id      serial      PRIMARY KEY,
                name    text        NOT NULL UNIQUE
            )"#,
            migrations_table_name
        )
    }
}

impl SupportsTransactionalDdl for PostgresConnection {
    #[inline]
    fn supports_transactional_ddl(&self) -> bool {
        true
    }
}

impl DatabaseConnection for PostgresConnection {
    fn batch_execute(&mut self, query: &str) -> StdResult<()> {
        self.client.batch_execute(query)?;
        Ok(())
    }

    fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64> {
        let stmt = merge_query_with_params(query, params);

        let res = self.client.execute(stmt.as_str(), &[])?;
        Ok(res)
    }

    fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>> {
        let stmt = merge_query_with_params(query, params);

        let res = self.client.query(stmt.as_str(), &[])?;

        let res = res
            .into_iter()
            .map(|row| {
                let column: String = row.get(0);
                vec![column]
            })
            .collect::<Vec<_>>();

        Ok(res)
    }
}
