use std::fmt;
use std::io;

pub type MigraResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailedOpenTransaction,
    FailedCommitTransaction,
    FailedRollbackTransaction,

    FailedCreateMigrationsTable,
    FailedApplySql,
    FailedInsertMigration,
    FailedDeleteMigration,
    FailedGetAppliedMigrations,

    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FailedOpenTransaction => fmt.write_str("Failed to open a transaction"),
            Error::FailedCommitTransaction => fmt.write_str("Failed to commit a transaction"),
            Error::FailedRollbackTransaction => fmt.write_str("Failed to rollback a transaction"),
            Error::FailedCreateMigrationsTable => {
                fmt.write_str("Failed to create a migrations table")
            }
            Error::FailedApplySql => fmt.write_str("Failed to apply sql"),
            Error::FailedInsertMigration => fmt.write_str("Failed to insert a migration"),
            Error::FailedDeleteMigration => fmt.write_str("Failed to delete a migration"),
            Error::FailedGetAppliedMigrations => fmt.write_str("Failed to get applied migrations"),
            Error::Io(ref error) => write!(fmt, "{}", error),
        }
    }
}

impl std::error::Error for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
