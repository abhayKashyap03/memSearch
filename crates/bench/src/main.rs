//! # bench
//!
//! Two kinds of measurement:
//!
//! - `ann`: vector index quality and speed. Recall@k vs. FlatIndex, QPS and
//!   p50/p99 latency, build time, memory, across an `ef_search` sweep.
//!   Datasets: start synthetic, then SIFT1M / GloVe from ann-benchmarks.com.
//! - `longmemeval`: end-to-end agent-memory accuracy on LongMemEval
//!   (Wu et al., 2024), plus tokens injected per turn, compared against
//!   baselines (full history in context, Letta, mem0).
//!
//! Usage (once implemented): `cargo run -p bench --release -- ann`
#![allow(dead_code)]

mod ann;
mod longmemeval;

fn main() {
    // TODO(you): parse argv, dispatch to ann::run or longmemeval::run.
    todo!()
}
