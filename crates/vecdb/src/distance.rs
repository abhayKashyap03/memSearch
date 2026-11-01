//! Distance functions. Start scalar and simple; later try making them fast
//! (loop unrolling, `std::simd`, or `std::arch` intrinsics) and measure the
//! difference in `bench`.
//!
//! Convention for the whole crate: **smaller distance = more similar**.

/// Which distance an index uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    /// Squared Euclidean distance. (Why squared? Think about what `sqrt`
    /// does to ordering, and what it costs.)
    L2,
    /// Cosine distance, i.e. `1 - cosine_similarity`.
    Cosine,
    /// Negative inner product, so that smaller still means closer.
    Dot,
}

impl Metric {
    /// Dispatch to the right function below.
    pub fn distance(&self, a: &[f32], b: &[f32]) -> f32 {
        let _ = (a, b);
        todo!("match on self and call the matching function")
    }
}

/// Squared L2 distance between `a` and `b`. Caller guarantees equal lengths.
pub fn l2_squared(a: &[f32], b: &[f32]) -> f32 {
    let _ = (a, b);
    todo!()
}

/// Inner product of `a` and `b`.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    let _ = (a, b);
    todo!()
}

/// `1 - cos(a, b)`. Decide what to return when either vector has zero norm,
/// and test it.
pub fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let _ = (a, b);
    todo!()
}

/// Scale `v` in place to unit length. Useful if you choose to pre-normalize
/// for cosine so search can use [`dot`] instead.
pub fn normalize(v: &mut [f32]) {
    let _ = v;
    todo!()
}
