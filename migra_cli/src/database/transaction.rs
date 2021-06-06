use super::client_rusqlite::Connection::AnyConnection;
use crate::errors::StdResult;

pub trait ManageTransaction {
    fn begin_transaction(&self, conn: &mut AnyConnection) -> migra::StdResult<()>;

    fn rollback_transaction(&self, conn: &mut AnyConnection) -> migra::StdResult<()>;

    fn commit_transaction(&self, conn: &mut AnyConnection) -> migra::StdResult<()>;
}

#[derive(Debug)]
pub struct TransactionManager;

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager
    }
}

impl ManageTransaction for TransactionManager {
    fn begin_transaction(&self, conn: &mut AnyConnection) -> migra::StdResult<()> {
        conn.batch_execute("BEGIN")
    }

    fn rollback_transaction(&self, conn: &mut AnyConnection) -> migra::StdResult<()> {
        conn.batch_execute("ROLLBACK")
    }

    fn commit_transaction(&self, conn: &mut AnyConnection) -> migra::StdResult<()> {
        conn.batch_execute("COMMIT")
    }
}
