use std::fmt::{self, Display, Formatter, write};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Abort,
    Config(String),
    Internal(String),
    Parse(String),
    ReadOnly,
    Serialization,
    Value(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Config(s) | Error::Internal(s) | Error::Parse(s) | Error::Value(s) => {
                write!(f, "{}", s)
            }
            Error::Abort => write!(f, "Operation aborted"),
            Error::Serialization => write!(f, "Serialization failure"),
            Error::ReadOnly => write!(f, "Read Only transaction"),
        }
    }
}

impl std::error::Error for Error {}
