use std::fmt;
use std::io;

pub type MigraResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
