use crate::config::SupportedDatabaseClient;
use crate::Config;
#[cfg(feature = "mysql")]
use migra::clients::MysqlClient;
#[cfg(feature = "postgres")]
use migra::clients::PostgresClient;
#[cfg(feature = "sqlite")]
use migra::clients::SqliteClient;
use migra::clients::{AnyClient, OpenDatabaseConnection};

pub fn create_client(
    client_kind: &SupportedDatabaseClient,
    connection_string: &str,
    migrations_table_name: &str,
) -> migra::Result<AnyClient> {
    let client: AnyClient = match client_kind {
        #[cfg(feature = "postgres")]
        SupportedDatabaseClient::Postgres => Box::new(PostgresClient::manual(
            connection_string,
            migrations_table_name,
        )?),
        #[cfg(feature = "mysql")]
        SupportedDatabaseClient::Mysql => Box::new(MysqlClient::manual(
            connection_string,
            migrations_table_name,
        )?),
        #[cfg(feature = "sqlite")]
        SupportedDatabaseClient::Sqlite => Box::new(SqliteClient::manual(
            connection_string,
            migrations_table_name,
        )?),
    };

    Ok(client)
}

pub fn create_client_from_config(config: &Config) -> migra::StdResult<AnyClient> {
    create_client(
        &config.database.client(),
        &config.database.connection_string()?,
        &config.migrations.table_name(),
    )
    .map_err(From::from)
}
