use std::fmt;

/// Errors returned by vecdb. Extend this as you go (I/O, corruption, ...).
#[derive(Debug)]
pub enum VecDbError {
    DimensionMismatch {
        expected: usize,
        got: usize,
    },
    NotFound(crate::VectorId),
    Io(std::io::Error),
    /// On-disk data failed a checksum or format check.
    Corrupt(String),
}

impl fmt::Display for VecDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: human-readable messages for each variant.
        write!(f, "{self:?}")
    }
}

impl std::error::Error for VecDbError {}

impl From<std::io::Error> for VecDbError {
    fn from(e: std::io::Error) -> Self {
        VecDbError::Io(e)
    }
}
