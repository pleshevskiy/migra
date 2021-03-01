use super::adapter::ToSqlParams;
use super::clients::*;
use crate::config::{DatabaseConfig, SupportedDatabaseClient};
use crate::error::StdResult;

pub type AnyConnection = Box<dyn DatabaseConnection>;

pub trait OpenDatabaseConnection: Sized {
    fn open(connection_string: &str) -> StdResult<Self>;
}

pub trait DatabaseConnection {
    fn batch_execute(&mut self, query: &str) -> StdResult<()>;

    fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64>;

    fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>>;
}

pub(crate) struct DatabaseConnectionManager {
    conn: AnyConnection,
}

impl DatabaseConnectionManager {
    pub fn connect_with_string(
        config: &DatabaseConfig,
        connection_string: &str,
    ) -> StdResult<Self> {
        let conn = match config.client()? {
            SupportedDatabaseClient::Postgres => PostgresConnection::open(&connection_string)?,
        };

        Ok(DatabaseConnectionManager {
            conn: Box::new(conn),
        })
    }

    pub fn connect(config: &DatabaseConfig) -> StdResult<Self> {
        let connection_string = config.connection_string()?;
        Self::connect_with_string(config, &connection_string)
    }

    pub fn connection(&mut self) -> &mut AnyConnection {
        &mut self.conn
    }
}
