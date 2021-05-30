use crate::error::MigraResult;
use crate::migration::{self, Migration};

pub trait ManageTransaction {
    fn begin_transaction(&mut self) -> MigraResult<()>;

    fn rollback_transaction(&mut self) -> MigraResult<()>;

    fn commit_transaction(&mut self) -> MigraResult<()>;
}

pub trait ManageMigrations {
    fn apply_sql(&mut self, sql_content: &str) -> MigraResult<()>;

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
