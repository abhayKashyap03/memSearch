//! Exact, brute-force index. O(n·d) per query. Slow, but always correct, which
//! makes it the reference that HNSW recall is measured against.

use crate::{Metric, SearchResult, VecDbError, VectorId, VectorIndex};

/// Stores every vector and compares the query against all of them.
///
/// Think about layout: a `Vec<Vec<f32>>` is easy; one contiguous `Vec<f32>`
/// is friendlier to the cache. Pick one, then try the other and measure.
pub struct FlatIndex {
    dim: usize,
    metric: Metric,
    // TODO: your storage fields.
}

impl FlatIndex {
    pub fn new(dim: usize, metric: Metric) -> Self {
        let _ = (dim, metric);
        todo!()
    }
}

impl VectorIndex for FlatIndex {
    fn dim(&self) -> usize {
        self.dim
    }

    fn metric(&self) -> Metric {
        self.metric
    }

    fn insert(&mut self, id: VectorId, vector: &[f32]) -> Result<(), VecDbError> {
        let _ = (id, vector);
        todo!()
    }

    fn remove(&mut self, id: VectorId) -> Result<bool, VecDbError> {
        let _ = id;
        todo!()
    }

    /// Hint: you don't need to sort all n distances to get the top k.
    fn search(&self, query: &[f32], k: usize) -> Result<Vec<SearchResult>, VecDbError> {
        let _ = (query, k);
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }
}
