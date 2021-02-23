use super::adapter::ToSqlParams;
use super::clients::*;
use crate::config::{DatabaseConfig, SupportedDatabaseClient};
use crate::error::StdResult;

pub trait OpenDatabaseConnection: Sized {
    fn open(connection_string: &str) -> StdResult<Self>;
}

pub trait DatabaseConnection {
    fn batch_execute(&mut self, query: &str) -> StdResult<()>;

    fn execute<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<u64>;

    fn query<'b>(&mut self, query: &str, params: ToSqlParams<'b>) -> StdResult<Vec<Vec<String>>>;
}

pub(crate) struct DatabaseConnectionManager {
    config: DatabaseConfig,
}

impl DatabaseConnectionManager {
    pub fn new(config: &DatabaseConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn connect_with_string(
        &self,
        connection_string: &str,
    ) -> StdResult<Box<dyn DatabaseConnection>> {
        let conn = match self.config.client()? {
            SupportedDatabaseClient::Postgres => PostgresConnection::open(&connection_string)?,
        };

        Ok(Box::new(conn))
    }

    pub fn connect(&self) -> StdResult<Box<dyn DatabaseConnection>> {
        let connection_string = self.config.connection_string()?;
        self.connect_with_string(&connection_string)
    }
}
