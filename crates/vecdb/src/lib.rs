//! # vecdb
//!
//! A vector index built from scratch. Milestones, in order:
//!
//! 1. [`distance`]: distance functions between vectors.
//! 2. [`flat::FlatIndex`]: exact brute-force k-NN. This is your ground truth for
//!    measuring recall of everything that comes after.
//! 3. [`hnsw::HnswIndex`]: approximate k-NN with a Hierarchical Navigable Small
//!    World graph (Malkov & Yashunin, 2016).
//! 4. [`storage`]: persist vectors and the graph to disk, survive a crash.
//!
//! Every index implements [`VectorIndex`], so `bench` and `memory` can swap
//! implementations without changing their code.

pub mod distance;
pub mod error;
pub mod flat;
pub mod hnsw;
pub mod storage;

pub use distance::Metric;
pub use error::VecDbError;

/// Stable identifier for a stored vector. Assigned by the caller, not the index.
pub type VectorId = u64;

/// One search hit.
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub id: VectorId,
    /// Distance under the index's [`Metric`]. Smaller is always closer, for
    /// every metric (decide how to make that true for dot product).
    pub distance: f32,
}

/// Common interface for every vector index in this crate.
pub trait VectorIndex {
    /// Number of dimensions every vector in this index must have.
    fn dim(&self) -> usize;

    /// Metric used to compare vectors.
    fn metric(&self) -> Metric;

    /// Insert `vector` under `id`.
    ///
    /// Must return [`VecDbError::DimensionMismatch`] if `vector.len() != self.dim()`.
    /// Decide and document what happens when `id` already exists
    /// (overwrite? error?), then write a test that pins that choice.
    fn insert(&mut self, id: VectorId, vector: &[f32]) -> Result<(), VecDbError>;

    /// Remove `id`. Returns `Ok(true)` if it existed, `Ok(false)` if not.
    fn remove(&mut self, id: VectorId) -> Result<bool, VecDbError>;

    /// Return up to `k` nearest neighbours of `query`, sorted closest first.
    ///
    /// Must return fewer than `k` results only when the index holds fewer than
    /// `k` vectors. Must return [`VecDbError::DimensionMismatch`] on a bad query.
    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, VecDbError>;

    /// Number of live (not removed) vectors.
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
