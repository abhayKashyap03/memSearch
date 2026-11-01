//! The memory store: what the MCP server calls.

use crate::{Embedder, MemoryError};
use lexical::InvertedIndex;
use vecdb::VectorIndex;

pub type MemoryId = u64;

/// One stored memory.
#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    pub id: MemoryId,
    pub text: String,
    /// Unix epoch milliseconds. Used for recency and time-scoped recall.
    pub created_at_ms: u64,
    /// Free-form tags (session id, topic, source, ...).
    pub tags: Vec<String>,
}

/// What the caller asks for.
#[derive(Debug, Clone)]
pub struct RecallQuery {
    pub text: String,
    pub k: usize,
    /// Only memories created at or after this time.
    pub since_ms: Option<u64>,
    /// Only memories carrying every one of these tags.
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecallHit {
    pub memory: Memory,
    pub score: f32,
}

/// Hybrid memory store, generic over the vector index and embedder so you can
/// run it on FlatIndex while HNSW is unfinished.
#[allow(dead_code)] // remove once the methods use these fields
pub struct MemoryStore<I: VectorIndex, E: Embedder> {
    vectors: I,
    keywords: InvertedIndex,
    embedder: E,
    // TODO: memory records by id, id allocation.
}

impl<I: VectorIndex, E: Embedder> MemoryStore<I, E> {
    /// Must fail if `vectors.dim() != embedder.dim()`.
    pub fn new(vectors: I, keywords: InvertedIndex, embedder: E) -> Result<Self, MemoryError> {
        let _ = (vectors, keywords, embedder);
        todo!()
    }

    /// Store `text`, index it in both retrievers, return its new id.
    pub fn remember(
        &mut self,
        text: &str,
        tags: &[String],
        now_ms: u64,
    ) -> Result<MemoryId, MemoryError> {
        let _ = (text, tags, now_ms);
        todo!()
    }

    /// Hybrid recall: query both retrievers, fuse with
    /// [`crate::fusion::reciprocal_rank_fusion`], apply filters, return top `k`.
    ///
    /// Design question to answer in code: do you filter before retrieval,
    /// after it, or over-fetch then filter? Each has a recall cost.
    pub fn recall(&self, query: &RecallQuery) -> Result<Vec<RecallHit>, MemoryError> {
        let _ = query;
        todo!()
    }

    /// Delete from every index. Returns whether it existed.
    pub fn forget(&mut self, id: MemoryId) -> Result<bool, MemoryError> {
        let _ = id;
        todo!()
    }

    pub fn get(&self, id: MemoryId) -> Option<&Memory> {
        let _ = id;
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
