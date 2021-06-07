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

pub fn with_transaction<TrxFnMut, Res>(
    client: &mut AnyClient,
    trx_fn: &mut TrxFnMut,
) -> migra::Result<Res>
where
    TrxFnMut: FnMut(&mut AnyClient) -> migra::Result<Res>,
{
    client
        .begin_transaction()
        .and_then(|_| trx_fn(client))
        .and_then(|res| client.commit_transaction().and(Ok(res)))
        .or_else(|err| client.rollback_transaction().and(Err(err)))
}

pub fn maybe_with_transaction<TrxFnMut, Res>(
    is_needed: bool,
    client: &mut AnyClient,
    trx_fn: &mut TrxFnMut,
) -> migra::Result<Res>
where
    TrxFnMut: FnMut(&mut AnyClient) -> migra::Result<Res>,
{
    if is_needed {
        with_transaction(client, trx_fn)
    } else {
        trx_fn(client)
    }
}
