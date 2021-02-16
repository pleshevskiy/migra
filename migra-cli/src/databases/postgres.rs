
use crate::migration::{MigrationNames, MigrationManager, is_migrations_table_not_found};
use postgres::{Client, NoTls};
use crate::config::Config;
use std::convert::TryFrom;
use crate::error::StdResult;
use crate::database::{TryFromSql, ToSql, DatabaseConnection};

pub struct PostgresConnection {
    client: Client,
}

impl TryFrom<&Config> for PostgresConnection {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        PostgresConnection::open(&config.database_connection_string()?)
    }
}

impl DatabaseConnection for PostgresConnection {
    type QueryResultRow = postgres::Row;
    type QueryResult = Vec<Self::QueryResultRow>;

    fn open(connection_string: &str) -> StdResult<Self> {
        let client = Client::connect(connection_string, NoTls)?;
        Ok(PostgresConnection { client })
    }

    fn batch_execute(&mut self, query: &str) -> StdResult<()> {
        self.client.batch_execute(query)?;
        Ok(())
    }

    fn execute<'b>(&mut self, query: &str, params: &'b [&'b dyn ToSql]) -> StdResult<u64> {
        let stmt = params
            .iter()
            .enumerate()
            .fold(query.to_string(), |acc, (i, p)| {
                str::replace(&acc, &format!("${}", i), &p.to_sql())
            });

        let res = self.client.execute(stmt.as_str(), &[])?;
        Ok(res)
    }

    fn query<'b, OutputItem>(
        &mut self,
        query: &str,
        params: &'b [&'b dyn ToSql],
    ) -> StdResult<Vec<OutputItem>>
    where
        OutputItem: ?Sized + TryFromSql<Self::QueryResultRow>,
    {
        let stmt = params
            .iter()
            .enumerate()
            .fold(query.to_string(), |acc, (i, p)| {
                str::replace(&acc, &format!("${}", i), &p.to_sql())
            });

        let res: Self::QueryResult = self.client.query(stmt.as_str(), &[])?;

        let res = res
            .into_iter()
            .map(OutputItem::try_from_sql)
            .collect::<Result<Vec<OutputItem>, _>>()?;

        Ok(res)
    }
}

impl MigrationNames for MigrationManager<PostgresConnection> {
    fn applied_migration_names(&mut self) -> StdResult<Vec<String>> {
        let res = self
            .conn
            .query(Self::APPLIED_MIGRATIONS_STMT, &[])
            .or_else(|e| {
                if is_migrations_table_not_found(&e) {
                    Ok(Vec::new())
                } else {
                    Err(e)
                }
            })?;

        Ok(res.into_iter().collect())
    }
}

impl TryFromSql<postgres::Row> for String {
    fn try_from_sql(row: postgres::Row) -> StdResult<Self> {
        let res: String = row.get(0);
        Ok(res)
    }
}
