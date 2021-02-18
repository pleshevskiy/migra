use crate::StdResult;

pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for &str {
    fn to_sql(&self) -> String {
        format!("'{}'", self)
    }
}

pub trait TryFromSql<QueryResultRow>: Sized {
    fn try_from_sql(row: QueryResultRow) -> StdResult<Self>;
}

pub trait OpenDatabaseConnection: Sized {
    fn open(connection_string: &str) -> StdResult<Self>;
}

pub trait DatabaseConnection {
    fn batch_execute(&mut self, query: &str) -> StdResult<()>;

    fn execute<'b>(&mut self, query: &str, params: &'b [&'b dyn ToSql]) -> StdResult<u64>;

    fn query<'b>(
        &mut self,
        query: &str,
        params: &'b [&'b dyn ToSql],
    ) -> StdResult<Vec<Vec<String>>>;
}
