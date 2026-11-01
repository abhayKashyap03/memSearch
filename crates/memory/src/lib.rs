//! # memory
//!
//! The agent-memory layer. Glues `vecdb` (semantic recall) and `lexical`
//! (exact-term recall) into one store, then fuses the two ranked lists.
//!
//! This is where the research questions live:
//! - recall on *implicit* references ("that bug from last week")
//! - fusion: which merge strategy wins on LongMemEval, and by how much
//! - what to return so the agent's context stays small (cost per turn)

pub mod embed;
pub mod fusion;
pub mod store;

pub use embed::Embedder;
pub use store::{Memory, MemoryId, MemoryStore, RecallHit, RecallQuery};

#[derive(Debug)]
pub enum MemoryError {
    Vector(vecdb::VecDbError),
    Embed(String),
    NotFound(MemoryId),
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for MemoryError {}

impl From<vecdb::VecDbError> for MemoryError {
    fn from(e: vecdb::VecDbError) -> Self {
        MemoryError::Vector(e)
    }
}
