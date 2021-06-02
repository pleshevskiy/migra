use crate::error::{Error, MigraResult, StdResult};
use crate::migration::{self, Migration};

pub trait BatchExecute {
    fn batch_execute(&mut self, sql: &str) -> StdResult<()>;
}

pub trait ManageTransaction: BatchExecute {
    fn begin_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("BEGIN")
            .map_err(|_| Error::FailedOpenTransaction)
    }

    fn rollback_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("ROLLBACK")
            .map_err(|_| Error::FailedRollbackTransaction)
    }

    fn commit_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("COMMIT")
            .map_err(|_| Error::FailedCommitTransaction)
    }
}

pub trait ManageMigrations: BatchExecute {
    fn apply_sql(&mut self, sql: &str) -> MigraResult<()> {
        self.batch_execute(sql).map_err(|_| Error::FailedApplySql)
    }

    fn create_migrations_table(&mut self) -> MigraResult<()>;

    fn insert_migration(&mut self, name: &str) -> MigraResult<u64>;

    fn delete_migration(&mut self, name: &str) -> MigraResult<u64>;

    fn applied_migrations(&mut self) -> MigraResult<migration::List>;

    fn apply_upgrade_migration(&mut self, migration: &Migration) -> MigraResult<()> {
        let content = migration.read_upgrade_migration_sql()?;

        self.create_migrations_table()?;
        self.apply_sql(&content)?;
        self.insert_migration(migration.name())?;

        Ok(())
    }

    fn apply_downgrade_migration(&mut self, migration: &Migration) -> MigraResult<()> {
        let content = migration.read_downgrade_migration_sql()?;

        self.apply_sql(&content)?;
        self.delete_migration(migration.name())?;

        Ok(())
    }
}
