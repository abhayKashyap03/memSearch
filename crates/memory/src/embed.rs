//! Turning text into vectors. The store doesn't care where embeddings come
//! from, so tests can use a fake and production can call a real model
//! (a local ONNX model, or an HTTP embedding API).

use crate::MemoryError;

pub trait Embedder {
    /// Output dimension. Must match the vector index.
    fn dim(&self) -> usize;

    /// Embed one piece of text.
    fn embed(&self, text: &str) -> Result<Vec<f32>, MemoryError>;

    /// Embed many. Override when your backend supports batching.
    fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, MemoryError> {
        texts.iter().map(|t| self.embed(t)).collect()
    }
}
