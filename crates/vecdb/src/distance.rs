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
        match self {
            Metric::L2 => return l2_squared(a, b),
            Metric::Cosine => return cosine_distance(a, b),
            Metric::Dot => return -1.0 * dot(a, b),
        };
    }
}

/// Squared L2 distance between `a` and `b`. Caller guarantees equal lengths.
pub fn l2_squared(a: &[f32], b: &[f32]) -> f32 {
    if magnitude(a).eq(&0.0) { return magnitude(b).powi(2); }
    if magnitude(b).eq(&0.0) { return magnitude(a).powi(2); }
    return a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f32>();
}

/// Inner product of `a` and `b`.
pub fn dot(a: &[f32], b: &[f32]) -> f32 {
    if magnitude(a).eq(&0.0) || magnitude(b).eq(&0.0) { return 0.0; }
    return a.iter().zip(b.iter()).map(|(x, y)| x*y).sum::<f32>();
}

/// `1 - cos(a, b)`. Decide what to return when either vector has zero norm,
/// and test it.
pub fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    if magnitude(a).eq(&0.0) || magnitude(b).eq(&0.0) { return 1.0; }
    return 1.0 - (dot(a, b) / (magnitude(a) * magnitude(b)));
}

/// Scale `v` in place to unit length. Useful if you choose to pre-normalize
/// for cosine so search can use [`dot`] instead.
pub fn normalize(v: &mut [f32]) {
    let norm = v.iter().map(|&x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        v.iter_mut().for_each(|x| *x /= norm);
    }
}

pub fn magnitude(v: &[f32]) -> f32 {
    return v.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
}
