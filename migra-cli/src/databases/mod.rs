mod postgres;

pub use self::postgres::*;

use crate::config::{DatabaseConfig, SupportedDatabaseClient};
use crate::database::{DatabaseConnection, OpenDatabaseConnection};
use crate::error::StdResult;

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
