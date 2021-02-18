use crate::database::{DatabaseConnection, OpenDatabaseConnection, ToSql};
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

impl DatabaseConnection for PostgresConnection {
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

    fn query<'b>(
        &mut self,
        query: &str,
        params: &'b [&'b dyn ToSql],
    ) -> StdResult<Vec<Vec<String>>> {
        let stmt = params
            .iter()
            .enumerate()
            .fold(query.to_string(), |acc, (i, p)| {
                str::replace(&acc, &format!("${}", i), &p.to_sql())
            });

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
