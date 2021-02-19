use crate::StdResult;

pub trait ToSql {
    fn to_sql(&self) -> String;
}

pub type ToSqlParams<'a> = &'a [&'a dyn ToSql];

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

    fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64>;

    fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>>;
}

pub(crate) fn merge_query_with_params(query: &str, params: ToSqlParams) -> String {
    params
        .iter()
        .enumerate()
        .fold(query.to_string(), |acc, (i, p)| {
            str::replace(&acc, &format!("${}", i + 1), &p.to_sql())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_query_with_params_successfully() {
        assert_eq!(
            merge_query_with_params("SELECT $1", &[&"foo"]),
            "SELECT 'foo'"
        );

        assert_eq!(
            merge_query_with_params("SELECT $1, $2", &[&"foo", &"bar"]),
            "SELECT 'foo', 'bar'"
        );

        assert_eq!(
            merge_query_with_params("SELECT $1, $1", &[&"foo"]),
            "SELECT 'foo', 'foo'"
        );
    }
}
