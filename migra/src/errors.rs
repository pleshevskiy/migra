use std::fmt;
use std::io;

/// A helper type for any standard error.
pub type StdError = Box<dyn std::error::Error + 'static + Sync + Send>;

/// A helper type for any result with standard error.
pub type StdResult<T> = Result<T, StdError>;

/// A helper type for any result with migra error.
pub type MigraResult<T> = Result<T, Error>;

/// Migra error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Represents database errors.
    Db(DbError),

    /// Represents standard input output errors.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Db(ref error) => write!(fmt, "{}", error),
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

impl Error {
    /// Creates a database error.
    #[must_use]
    pub fn db(origin: StdError, kind: DbKind) -> Self {
        Error::Db(DbError { kind, origin })
    }
}

/// All kinds of errors with witch this crate works.
#[derive(Debug)]
#[non_exhaustive]
pub enum DbKind {
    /// Failed to database connection.
    DatabaseConnection,

    /// Failed to open transaction.
    OpenTransaction,

    /// Failed to commit transaction.
    CommitTransaction,

    /// Failed to rollback transaction.
    RollbackTransaction,

    /// Failed to create a migrations table.
    CreateMigrationsTable,

    /// Failed to apply SQL.
    ApplySql,

    /// Failed to insert a migration.
    InsertMigration,

    /// Failed to delete a migration.
    DeleteMigration,

    /// Failed to get applied migrations.
    GetAppliedMigrations,
}

impl fmt::Display for DbKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbKind::DatabaseConnection => fmt.write_str("Failed database connection"),
            DbKind::OpenTransaction => fmt.write_str("Failed to open a transaction"),
            DbKind::CommitTransaction => fmt.write_str("Failed to commit a transaction"),
            DbKind::RollbackTransaction => fmt.write_str("Failed to rollback a transaction"),
            DbKind::CreateMigrationsTable => fmt.write_str("Failed to create a migrations table"),
            DbKind::ApplySql => fmt.write_str("Failed to apply sql"),
            DbKind::InsertMigration => fmt.write_str("Failed to insert a migration"),
            DbKind::DeleteMigration => fmt.write_str("Failed to delete a migration"),
            DbKind::GetAppliedMigrations => fmt.write_str("Failed to get applied migrations"),
        }
    }
}

/// Represents database error.
#[derive(Debug)]
pub struct DbError {
    kind: DbKind,
    origin: StdError,
}

impl fmt::Display for DbError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{} - {}", &self.kind, &self.origin)
    }
}

impl DbError {
    /// Returns database error kind.
    #[must_use]
    pub fn kind(&self) -> &DbKind {
        &self.kind
    }

    /// Returns origin database error.
    #[must_use]
    pub fn origin(&self) -> &StdError {
        &self.origin
    }
}
