use std::error::Error as StdError;
use std::fmt;
use std::num::ParseIntError;

/// An error returned during parsing of [Versions](crate::Version) or [Ranges](crate::Range).
/// Use the [kind](Error::kind) method to get the type of error.
#[derive(Debug)]
pub struct Error {
    _kind: ErrorKind,
}

impl Error {
    /// Construct a new error from a specific [kind](Error::kind)
    pub fn new(kind: ErrorKind) -> Self {
        Error { _kind: kind }
    }

    /// Get the kind of the error
    pub fn kind(&self) -> ErrorKind {
        self._kind.clone()
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::new(ErrorKind::ParseInt(e))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.kind())
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidComparator(String),
    InvalidRange(String),
    ParseInt(ParseIntError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ErrorKind::InvalidComparator(comp) => format!("Invalid comparator {}", comp),
            ErrorKind::InvalidRange(range) => format!("Invalid range {}", range),
            ErrorKind::ParseInt(e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}
