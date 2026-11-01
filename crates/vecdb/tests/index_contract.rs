//! Behaviour every VectorIndex must satisfy. FlatIndex first, then HNSW.
//! Run with: cargo test -p vecdb -- --ignored

use vecdb::flat::FlatIndex;
use vecdb::hnsw::{HnswConfig, HnswIndex};
use vecdb::{Metric, VecDbError, VectorIndex};

fn hnsw(dim: usize) -> HnswIndex {
    HnswIndex::new(
        dim,
        Metric::L2,
        HnswConfig {
            m: 16,
            m_max0: 32,
            ef_construction: 200,
            ef_search: 64,
            seed: 42,
        },
    )
}

/// Shared checks, run against any index.
fn basic_contract(mut idx: impl VectorIndex) {
    assert!(idx.is_empty());
    idx.insert(1, &[0.0, 0.0]).unwrap();
    idx.insert(2, &[1.0, 0.0]).unwrap();
    idx.insert(3, &[5.0, 5.0]).unwrap();
    assert_eq!(idx.len(), 3);

    let hits = idx.search(&[0.1, 0.0], 2).unwrap();
    let ids: Vec<_> = hits.iter().map(|h| h.id).collect();
    assert_eq!(ids, vec![1, 2], "closest first");
    assert!(hits[0].distance <= hits[1].distance);

    // k larger than the index returns everything.
    assert_eq!(idx.search(&[0.0, 0.0], 10).unwrap().len(), 3);

    // Wrong dimension is an error, not a panic.
    assert!(matches!(
        idx.insert(4, &[1.0, 2.0, 3.0]),
        Err(VecDbError::DimensionMismatch {
            expected: 2,
            got: 3
        })
    ));
    assert!(idx.search(&[1.0], 1).is_err());

    // Removed vectors never come back.
    assert!(idx.remove(1).unwrap());
    assert!(!idx.remove(1).unwrap());
    assert_eq!(idx.len(), 2);
    let ids: Vec<_> = idx
        .search(&[0.0, 0.0], 3)
        .unwrap()
        .iter()
        .map(|h| h.id)
        .collect();
    assert!(!ids.contains(&1));
}

#[test]
#[ignore = "implement FlatIndex"]
fn flat_basic_contract() {
    basic_contract(FlatIndex::new(2, Metric::L2));
}

#[test]
#[ignore = "implement HnswIndex"]
fn hnsw_basic_contract() {
    basic_contract(hnsw(2));
}

#[test]
#[ignore = "implement HnswIndex (needs FlatIndex as ground truth)"]
fn hnsw_recall_at_10_is_high() {
    // TODO(you): generate ~10_000 random vectors (dim 32) with a seeded RNG
    // you write yourself, insert into both FlatIndex and HnswIndex, run ~100
    // queries, and compute recall@10 = |hnsw ∩ flat| / 10 averaged over
    // queries. Target: >= 0.95 at ef_search = 64.
    let recall_at_10 = measure_recall_at_10(FlatIndex::new(32, Metric::L2), hnsw(32));
    assert!(recall_at_10 >= 0.95, "recall@10 = {recall_at_10}");
}

/// TODO(you): fill in. Returns mean recall@10 of `approx` against `exact`.
fn measure_recall_at_10(exact: impl VectorIndex, approx: impl VectorIndex) -> f32 {
    let _ = (exact, approx);
    todo!()
}
