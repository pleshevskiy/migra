use crate::config::SupportedDatabaseClient;
#[cfg(feature = "mysql")]
use migra::clients::MysqlClient;
#[cfg(feature = "postgres")]
use migra::clients::PostgresClient;
#[cfg(feature = "sqlite")]
use migra::clients::SqliteClient;
use migra::clients::{AnyClient, OpenDatabaseConnection};

pub fn create(
    client_kind: &SupportedDatabaseClient,
    connection_string: &str,
) -> migra::Result<AnyClient> {
    let client: AnyClient = match client_kind {
        #[cfg(feature = "postgres")]
        SupportedDatabaseClient::Postgres => Box::new(PostgresClient::new(&connection_string)?),
        #[cfg(feature = "mysql")]
        SupportedDatabaseClient::Mysql => Box::new(MysqlClient::new(&connection_string)?),
        #[cfg(feature = "sqlite")]
        SupportedDatabaseClient::Sqlite => Box::new(SqliteClient::new(&connection_string)?),
    };

    Ok(client)
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
