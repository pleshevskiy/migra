use std::fmt;
use std::io;

pub type StdError = Box<dyn std::error::Error + 'static + Sync + Send>;
pub type StdResult<T> = Result<T, StdError>;
pub type MigraResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Db(DbError),
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
    #[must_use]
    pub fn db(origin: StdError, kind: DbKind) -> Self {
        Error::Db(DbError { kind, origin })
    }
}

#[derive(Debug)]
pub enum DbKind {
    DatabaseConnection,

    OpenTransaction,
    CommitTransaction,
    RollbackTransaction,

    CreateMigrationsTable,
    ApplySql,
    InsertMigration,
    DeleteMigration,
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
