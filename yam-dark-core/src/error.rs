use core::fmt;
use core::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    Eof,
    Syntax,
    InvalidUtf8,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    /// Type of error
    error: ErrorType,
}

impl Error {
    pub fn generic(t: ErrorType) -> Self {
        Self { error: t }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.error)
    }
}
