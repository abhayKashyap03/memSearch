//! End-to-end behaviour of the hybrid store, using a fake embedder.
//! Run with: cargo test -p memory -- --ignored

use lexical::{Bm25Params, InvertedIndex};
use memory::{Embedder, MemoryError, MemoryStore, RecallQuery};
use vecdb::flat::FlatIndex;
use vecdb::Metric;

/// Deterministic fake: TODO(you) make it map text to a vector so that texts
/// sharing words land near each other (e.g. hashed bag of words).
struct FakeEmbedder;

impl Embedder for FakeEmbedder {
    fn dim(&self) -> usize {
        64
    }
    fn embed(&self, text: &str) -> Result<Vec<f32>, MemoryError> {
        let _ = text;
        todo!()
    }
}

fn store() -> MemoryStore<FlatIndex, FakeEmbedder> {
    MemoryStore::new(
        FlatIndex::new(64, Metric::Cosine),
        InvertedIndex::new(Bm25Params { k1: 1.2, b: 0.75 }),
        FakeEmbedder,
    )
    .unwrap()
}

fn q(text: &str, k: usize) -> RecallQuery {
    RecallQuery {
        text: text.into(),
        k,
        since_ms: None,
        tags: vec![],
    }
}

#[test]
#[ignore = "implement MemoryStore"]
fn remember_then_recall_exact_identifier() {
    let mut s = store();
    s.remember("deploy failed with error E1234 on staging", &[], 1)
        .unwrap();
    s.remember("user prefers dark mode", &[], 2).unwrap();
    let hits = s.recall(&q("E1234", 1)).unwrap();
    assert!(hits[0].memory.text.contains("E1234"));
}

#[test]
#[ignore = "implement MemoryStore"]
fn forget_removes_from_recall() {
    let mut s = store();
    let id = s
        .remember("secret project codename falcon", &[], 1)
        .unwrap();
    assert!(s.forget(id).unwrap());
    assert!(s.get(id).is_none());
    assert!(s
        .recall(&q("falcon", 5))
        .unwrap()
        .iter()
        .all(|h| h.memory.id != id));
}

#[test]
#[ignore = "implement MemoryStore"]
fn filters_by_time_and_tag() {
    let mut s = store();
    s.remember("standup notes monday", &["work".into()], 100)
        .unwrap();
    s.remember("standup notes friday", &["work".into()], 500)
        .unwrap();
    s.remember("standup comedy tickets", &["personal".into()], 600)
        .unwrap();
    let hits = s
        .recall(&RecallQuery {
            text: "standup".into(),
            k: 10,
            since_ms: Some(200),
            tags: vec!["work".into()],
        })
        .unwrap();
    assert_eq!(hits.len(), 1);
    assert!(hits[0].memory.text.contains("friday"));
}
