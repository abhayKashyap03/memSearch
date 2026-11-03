//! Expected behaviour of the distance functions.
//! Run with: cargo test -p vecdb -- --ignored
//! Remove each #[ignore] as you implement the function it covers.

use vecdb::distance::{cosine_distance, dot, l2_squared, normalize};
use vecdb::Metric;

const EPS: f32 = 1e-6;

#[test]
fn l2_squared_known_values() {
    assert!((l2_squared(&[0.0, 0.0], &[3.0, 4.0]) - 25.0).abs() < EPS);
    assert_eq!(l2_squared(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.0]), 0.0);
}

#[test]
fn dot_known_values() {
    assert!((dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]) - 32.0).abs() < EPS);
}

#[test]
fn cosine_distance_known_values() {
    // Same direction: 0. Orthogonal: 1. Opposite: 2.
    assert!(cosine_distance(&[1.0, 0.0], &[5.0, 0.0]).abs() < EPS);
    assert!((cosine_distance(&[1.0, 0.0], &[0.0, 1.0]) - 1.0).abs() < EPS);
    assert!((cosine_distance(&[1.0, 0.0], &[-1.0, 0.0]) - 2.0).abs() < EPS);
}

#[test]
fn normalize_gives_unit_length() {
    let mut v = vec![3.0, 4.0];
    normalize(&mut v);
    assert!((dot(&v, &v) - 1.0).abs() < EPS);
}

#[test]
fn smaller_means_closer_for_every_metric() {
    let q = [1.0, 0.0];
    let near = [0.9, 0.1];
    let far = [-1.0, 0.0];
    for m in [Metric::L2, Metric::Cosine, Metric::Dot] {
        assert!(
            m.distance(&q, &near) < m.distance(&q, &far),
            "{m:?} violates smaller = closer"
        );
    }
}
