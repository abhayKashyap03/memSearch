//! On-disk persistence. Do this after HNSW works in memory.
//!
//! A reasonable path:
//! 1. Write-ahead log: append every insert/remove as a record with a length
//!    prefix and checksum; `fsync` on commit; replay on open.
//! 2. Snapshot: periodically write the full index to a file, then reset
//!    the WAL. Write to a temp file and `rename` so a crash never leaves a
//!    half-written snapshot.
//! 3. Later: `mmap` the vector data instead of reading it all into memory.

use crate::{VecDbError, VectorId};
use std::path::Path;

/// One logged mutation.
#[derive(Debug, Clone, PartialEq)]
pub enum WalRecord {
    Insert { id: VectorId, vector: Vec<f32> },
    Remove { id: VectorId },
}

/// Append-only log of mutations.
pub struct Wal {
    // TODO: file handle, buffering, etc.
}

impl Wal {
    /// Open or create the log at `path`.
    pub fn open(path: &Path) -> Result<Self, VecDbError> {
        let _ = path;
        todo!()
    }

    /// Append `record`. Durable only after [`Wal::sync`].
    pub fn append(&mut self, record: &WalRecord) -> Result<(), VecDbError> {
        let _ = record;
        todo!()
    }

    /// Flush and fsync.
    pub fn sync(&mut self) -> Result<(), VecDbError> {
        todo!()
    }

    /// Read back every complete record in order. A torn final record (crash
    /// mid-write) must be ignored, not reported as an error. Corruption in
    /// the middle of the log must return [`VecDbError::Corrupt`].
    pub fn replay(path: &Path) -> Result<Vec<WalRecord>, VecDbError> {
        let _ = path;
        todo!()
    }
}
