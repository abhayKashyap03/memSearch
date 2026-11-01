//! Hierarchical Navigable Small World graph.
//!
//! Read first: Malkov & Yashunin, "Efficient and robust approximate nearest
//! neighbor search using Hierarchical Navigable Small World graphs" (2016),
//! especially Algorithms 1 to 5. Implement them in that order.
//!
//! Suggested internal pieces (all yours to design):
//! - node storage: vector + per-layer neighbour lists
//! - random level assignment for new nodes
//! - `search_layer`: greedy best-first search with a candidate heap and a
//!   result heap of size `ef`
//! - neighbour selection (simple, then the heuristic from the paper)
//! - entry point tracking
//! - deletes (tombstones are the easy first version)

use crate::{Metric, SearchResult, VecDbError, VectorId, VectorIndex};

/// Tuning knobs. Names follow the paper.
#[derive(Debug, Clone)]
pub struct HnswConfig {
    /// Max neighbours per node on layers above 0.
    pub m: usize,
    /// Max neighbours per node on layer 0 (paper suggests 2·M).
    pub m_max0: usize,
    /// Candidate list size while building.
    pub ef_construction: usize,
    /// Candidate list size while searching. Must be >= k.
    pub ef_search: usize,
    /// Seed for level assignment, so builds are reproducible in tests.
    pub seed: u64,
}

pub struct HnswIndex {
    dim: usize,
    metric: Metric,
    config: HnswConfig,
    // TODO: graph fields.
}

impl HnswIndex {
    pub fn new(dim: usize, metric: Metric, config: HnswConfig) -> Self {
        let _ = (dim, metric, config);
        todo!()
    }

    /// Change `ef_search` without rebuilding. Lets `bench` sweep the
    /// recall/latency curve.
    pub fn set_ef_search(&mut self, ef: usize) {
        self.config.ef_search = ef;
    }
}

impl VectorIndex for HnswIndex {
    fn dim(&self) -> usize {
        self.dim
    }

    fn metric(&self) -> Metric {
        self.metric
    }

    fn insert(&mut self, id: VectorId, vector: &[f32]) -> Result<(), VecDbError> {
        let _ = (id, vector);
        todo!("paper Algorithm 1")
    }

    fn remove(&mut self, id: VectorId) -> Result<bool, VecDbError> {
        let _ = id;
        todo!()
    }

    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, VecDbError> {
        let _ = (query, k);
        todo!("paper Algorithm 5")
    }

    fn len(&self) -> usize {
        todo!()
    }
}
