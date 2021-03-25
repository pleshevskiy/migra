use super::adapter::ToSqlParams;
use super::clients::*;
use crate::config::{DatabaseConfig, SupportedDatabaseClient};
use crate::error::StdResult;

pub type AnyConnection = Box<dyn DatabaseConnection>;

pub trait OpenDatabaseConnection: Sized {
    fn open(connection_string: &str) -> StdResult<Self>;
}

pub trait DatabaseConnection {
    fn migration_table_stmt(&self) -> String;

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
        let conn: AnyConnection = match config.client() {
            #[cfg(feature = "postgres")]
            SupportedDatabaseClient::Postgres => {
                Box::new(PostgresConnection::open(&connection_string)?)
            }
            #[cfg(feature = "mysql")]
            SupportedDatabaseClient::Mysql => Box::new(MySqlConnection::open(&connection_string)?),
        };

        Ok(DatabaseConnectionManager { conn })
    }

    pub fn connect(config: &DatabaseConfig) -> StdResult<Self> {
        let connection_string = config.connection_string()?;
        Self::connect_with_string(config, &connection_string)
    }

    pub fn connection(&mut self) -> &mut AnyConnection {
        &mut self.conn
    }
}
