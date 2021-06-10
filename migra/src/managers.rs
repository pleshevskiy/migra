use crate::errors::{DbKind, Error, MigraResult, StdResult};
use crate::migration;

pub trait BatchExecute {
    fn batch_execute(&mut self, sql: &str) -> StdResult<()>;
}

pub trait ManageTransaction: BatchExecute {
    fn begin_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("BEGIN")
            .map_err(|err| Error::db(err, DbKind::OpenTransaction))
    }

    fn rollback_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("ROLLBACK")
            .map_err(|err| Error::db(err, DbKind::RollbackTransaction))
    }

    fn commit_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("COMMIT")
            .map_err(|err| Error::db(err, DbKind::CommitTransaction))
    }
}

pub trait ManageMigrations: BatchExecute {
    fn apply_sql(&mut self, sql: &str) -> MigraResult<()> {
        self.batch_execute(sql)
            .map_err(|err| Error::db(err, DbKind::ApplySql))
    }

    fn create_migrations_table(&mut self) -> MigraResult<()>;

    fn insert_migration(&mut self, name: &str) -> MigraResult<u64>;

    fn delete_migration(&mut self, name: &str) -> MigraResult<u64>;

    fn get_applied_migrations(&mut self) -> MigraResult<migration::List>;

    fn run_upgrade_migration(&mut self, name: &str, content: &str) -> MigraResult<()> {
        self.apply_sql(content)?;
        self.insert_migration(name)?;
        Ok(())
    }

    fn run_downgrade_migration(&mut self, name: &str, content: &str) -> MigraResult<()> {
        self.apply_sql(content)?;
        self.delete_migration(name)?;
        Ok(())
    }
}
