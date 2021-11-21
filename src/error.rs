/// An error returned during parsing of [Versions](crate::Version) or [Ranges](crate::Range).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("invalid comparator: {0}")]
    InvalidComparator(String),

    #[error("invalid range: {0}")]
    InvalidRange(String),
}
