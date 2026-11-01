//! Expected behaviour of tokenizer and BM25 search.
//! Run with: cargo test -p lexical -- --ignored

use lexical::{tokenize, Bm25Params, InvertedIndex};

fn index() -> InvertedIndex {
    // TODO(you): replace with the defaults you settle on.
    InvertedIndex::new(Bm25Params { k1: 1.2, b: 0.75 })
}

#[test]
#[ignore = "implement tokenize"]
fn tokenize_lowercases_and_splits() {
    assert_eq!(tokenize("Hello, World! v2"), vec!["hello", "world", "v2"]);
    assert!(tokenize("  ...  ").is_empty());
}

#[test]
#[ignore = "implement InvertedIndex"]
fn only_matching_docs_are_returned() {
    let mut idx = index();
    idx.add(1, "the cat sat on the mat");
    idx.add(2, "dogs chase cars");
    let hits = idx.search("cat", 10);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, 1);
    assert!(idx.search("zebra", 10).is_empty());
}

#[test]
#[ignore = "implement InvertedIndex"]
fn rare_terms_outweigh_common_terms() {
    let mut idx = index();
    idx.add(1, "meeting notes alpha");
    idx.add(2, "meeting notes beta");
    idx.add(3, "meeting notes gamma");
    idx.add(4, "meeting kubernetes");
    // "meeting" is everywhere, "kubernetes" is rare: doc 4 must win.
    assert_eq!(idx.search("meeting kubernetes", 4)[0].id, 4);
}

#[test]
#[ignore = "implement InvertedIndex"]
fn shorter_doc_wins_on_equal_tf() {
    let mut idx = index();
    idx.add(1, "rust");
    idx.add(2, "rust is a language with many many many words in it");
    idx.add(3, "unrelated filler text");
    assert_eq!(idx.search("rust", 2)[0].id, 1);
}

#[test]
#[ignore = "implement InvertedIndex"]
fn remove_and_replace() {
    let mut idx = index();
    idx.add(1, "apple");
    idx.add(1, "banana"); // replaces
    assert!(idx.search("apple", 10).is_empty());
    assert_eq!(idx.search("banana", 10)[0].id, 1);
    assert!(idx.remove(1));
    assert!(!idx.remove(1));
    assert!(idx.is_empty());
}
