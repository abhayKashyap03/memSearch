//! # lexical
//!
//! Keyword search from scratch, so the memory layer can catch exact names,
//! IDs and rare terms that embeddings blur together.
//!
//! Milestones:
//! 1. [`tokenize`]: text to terms.
//! 2. [`InvertedIndex`]: term to postings (doc id, term frequency), plus the
//!    per-document lengths BM25 needs.
//! 3. [`InvertedIndex::search`]: Okapi BM25 ranking. Derive the formula from
//!    Robertson & Zaragoza, "The Probabilistic Relevance Framework: BM25 and
//!    Beyond" (2009), section 3.

/// Document identifier, chosen by the caller. The memory layer uses the same
/// id for a memory in both the lexical and vector index.
pub type DocId = u64;

/// One ranked hit. Larger score = more relevant (the opposite of vecdb's
/// distance; the fusion step in `memory` must cope with that).
#[derive(Debug, Clone, PartialEq)]
pub struct ScoredDoc {
    pub id: DocId,
    pub score: f32,
}

/// BM25 free parameters. Look up the usual defaults and explain to yourself
/// what each one controls before picking values.
#[derive(Debug, Clone, Copy)]
pub struct Bm25Params {
    /// Term-frequency saturation.
    pub k1: f32,
    /// Document-length normalisation, in [0, 1].
    pub b: f32,
}

/// Split `text` into normalised terms.
///
/// Start simple: lowercase, split on non-alphanumeric. Later options to
/// evaluate with real data: stop words, stemming, keeping `snake_case` and
/// `CamelCase` identifiers intact, unicode normalisation.
pub fn tokenize(text: &str) -> Vec<String> {
    let _ = text;
    todo!()
}

/// In-memory inverted index.
pub struct InvertedIndex {
    params: Bm25Params,
    // TODO: postings, doc lengths, total length / doc count for avgdl.
}

impl InvertedIndex {
    pub fn new(params: Bm25Params) -> Self {
        let _ = params;
        todo!()
    }

    pub fn params(&self) -> Bm25Params {
        self.params
    }

    /// Index `text` under `id`. Re-adding an existing id replaces the old text.
    pub fn add(&mut self, id: DocId, text: &str) {
        let _ = (id, text);
        todo!()
    }

    /// Remove `id`. Returns whether it existed. Corpus statistics (doc count,
    /// average length) must update too.
    pub fn remove(&mut self, id: DocId) -> bool {
        let _ = id;
        todo!()
    }

    /// Top `k` documents for `query` by BM25, highest score first. Documents
    /// that share no term with the query are not returned.
    pub fn search(&self, query: &str, k: usize) -> Vec<ScoredDoc> {
        let _ = (query, k);
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
