use crate::errors::{DbKind, Error, MigraResult, StdResult};
use crate::migration;

/// Used to execute SQL.
///
/// Is a super trait for managers.
pub trait BatchExecute {
    /// Executes sql via original database client
    fn batch_execute(&mut self, sql: &str) -> StdResult<()>;
}

/// Used to manage transaction in the database connection.
pub trait ManageTransaction: BatchExecute {
    /// Opens transaction in database connection.
    fn begin_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("BEGIN")
            .map_err(|err| Error::db(err, DbKind::OpenTransaction))
    }

    /// Cancels (Rollbacks) transaction in database connection.
    fn rollback_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("ROLLBACK")
            .map_err(|err| Error::db(err, DbKind::RollbackTransaction))
    }

    /// Apply (Commit) transaction in database connection.
    fn commit_transaction(&mut self) -> MigraResult<()> {
        self.batch_execute("COMMIT")
            .map_err(|err| Error::db(err, DbKind::CommitTransaction))
    }
}

/// Used to manage migrations in the database connection.
pub trait ManageMigrations: BatchExecute {
    /// Applies SQL. Similar to [`BatchExecute`], but returns migra [Error].
    ///
    /// [BatchExecute]: managers::BatchExecute
    fn apply_sql(&mut self, sql: &str) -> MigraResult<()> {
        self.batch_execute(sql)
            .map_err(|err| Error::db(err, DbKind::ApplySql))
    }

    /// Creates migration table.
    fn create_migrations_table(&mut self) -> MigraResult<()>;

    /// Inserts new migration to table.
    fn insert_migration(&mut self, name: &str) -> MigraResult<u64>;

    /// Deletes migration from table.
    fn delete_migration(&mut self, name: &str) -> MigraResult<u64>;

    /// Get applied migrations from table.
    fn get_applied_migrations(&mut self) -> MigraResult<migration::List>;

    /// Applies SQL to upgrade database schema and inserts new migration to table.
    ///
    /// **Note:** Must be run in a transaction otherwise if the migration causes any
    /// error the data in the database may be inconsistent.
    fn run_upgrade_migration(&mut self, name: &str, content: &str) -> MigraResult<()> {
        self.apply_sql(content)?;
        self.insert_migration(name)?;
        Ok(())
    }

    /// Applies SQL to downgrade database schema and deletes migration from table.
    ///
    /// **Note:** Must be run in a transaction otherwise if the migration causes any
    /// error the data in the database may be inconsistent.
    fn run_downgrade_migration(&mut self, name: &str, content: &str) -> MigraResult<()> {
        self.apply_sql(content)?;
        self.delete_migration(name)?;
        Ok(())
    }
}
