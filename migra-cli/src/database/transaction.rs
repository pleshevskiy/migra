use super::connection::AnyConnection;
use crate::error::StdResult;

pub trait ManageTransaction {
    fn begin_transaction(&self, conn: &mut AnyConnection) -> StdResult<()>;

    fn rollback_transaction(&self, conn: &mut AnyConnection) -> StdResult<()>;

    fn commit_transaction(&self, conn: &mut AnyConnection) -> StdResult<()>;
}

#[derive(Debug)]
pub struct TransactionManager;

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager
    }
}

impl ManageTransaction for TransactionManager {
    fn begin_transaction(&self, conn: &mut AnyConnection) -> StdResult<()> {
        conn.batch_execute("BEGIN")
    }

    fn rollback_transaction(&self, conn: &mut AnyConnection) -> StdResult<()> {
        conn.batch_execute("ROLLBACK")
    }

    fn commit_transaction(&self, conn: &mut AnyConnection) -> StdResult<()> {
        conn.batch_execute("COMMIT")
    }
}

pub fn with_transaction<TrxFnMut, Res>(
    conn: &mut AnyConnection,
    trx_fn: &mut TrxFnMut,
) -> StdResult<Res>
where
    TrxFnMut: FnMut(&mut AnyConnection) -> StdResult<Res>,
{
    let transaction_manager = TransactionManager::new();
    transaction_manager
        .begin_transaction(conn)
        .and_then(|_| trx_fn(conn))
        .and_then(|res| transaction_manager.commit_transaction(conn).and(Ok(res)))
        .or_else(|err| transaction_manager.rollback_transaction(conn).and(Err(err)))
}
