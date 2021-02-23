use crate::error::StdResult;

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
