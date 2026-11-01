//! ANN benchmark harness.

use vecdb::{SearchResult, VectorIndex};

/// One row of the results table.
#[derive(Debug, Clone)]
pub struct AnnReport {
    pub index_name: String,
    pub ef_search: usize,
    pub recall_at_k: f32,
    pub qps: f64,
    pub p50_us: f64,
    pub p99_us: f64,
    pub build_secs: f64,
}

/// Fraction of `truth` ids present in `got`.
pub fn recall(got: &[SearchResult], truth: &[SearchResult]) -> f32 {
    let _ = (got, truth);
    todo!()
}

/// Build `index` from `base`, run every query, and report against
/// `ground_truth` (exact top-k per query, from FlatIndex or the dataset).
pub fn evaluate(
    name: &str,
    index: &mut dyn VectorIndex,
    base: &[Vec<f32>],
    queries: &[Vec<f32>],
    ground_truth: &[Vec<SearchResult>],
    k: usize,
) -> AnnReport {
    let _ = (name, index, base, queries, ground_truth, k);
    todo!()
}

/// Read `.fvecs` (the SIFT/GIST format): each vector is a little-endian i32
/// dimension followed by that many f32s.
pub fn read_fvecs(path: &std::path::Path) -> std::io::Result<Vec<Vec<f32>>> {
    let _ = path;
    todo!()
}

pub fn run(args: &[String]) {
    let _ = args;
    todo!()
}
