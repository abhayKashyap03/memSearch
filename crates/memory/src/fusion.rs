//! Merging ranked lists from different retrievers.
//!
//! vecdb returns distances (smaller = better); lexical returns BM25 scores
//! (larger = better, unbounded). Raw values are not comparable, which is why
//! rank-based fusion exists.
//!
//! Implement Reciprocal Rank Fusion first (Cormack, Clarke & Büttcher, SIGIR
//! 2009). Then try at least one score-based alternative and compare them in
//! `bench`. That comparison is a write-up worth publishing.

use crate::MemoryId;

/// One retriever's output: ids in rank order, best first.
pub type RankedList = Vec<MemoryId>;

/// A fused hit.
#[derive(Debug, Clone, PartialEq)]
pub struct Fused {
    pub id: MemoryId,
    pub score: f32,
}

/// Reciprocal Rank Fusion over any number of ranked lists, with constant `k`
/// (read the paper for what it does and the value it used).
///
/// Output: every id that appears in any list, highest fused score first.
/// Ties must break deterministically (pick a rule and test it).
pub fn reciprocal_rank_fusion(lists: &[RankedList], k: f32) -> Vec<Fused> {
    let _ = (lists, k);
    todo!()
}

/// Weighted variant: `weights[i]` scales list `i`'s contribution. Lets you
/// tune how much keyword vs. semantic recall matters.
pub fn weighted_rrf(lists: &[RankedList], weights: &[f32], k: f32) -> Vec<Fused> {
    let _ = (lists, weights, k);
    todo!()
}
