use std::error;
use std::fmt;
use std::io;
use std::mem;
use std::result;

pub type StdResult<T> = result::Result<T, Box<dyn std::error::Error>>;
pub type MigraResult<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RootNotFound,
    MissedEnvVar(String),

    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::RootNotFound => fmt.write_str("Cannot find root directory"),
            Error::MissedEnvVar(ref name) => {
                write!(fmt, r#"Missed "{}" environment variable"#, name)
            }
            Error::IoError(ref error) => write!(fmt, "{}", error),
        }
    }
}

impl error::Error for Error {}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
