# memSearch

A vector database written from scratch in Rust, and an agent-memory MCP server built on top of it.

**Status: skeleton.** Every function is a `todo!()` stub with doc comments describing what it must do. Behaviour is pinned down by `#[ignore]`d tests. The work is filling them in.

## Goal

Agents forget. The usual fix (dump history into a vector store, retrieve top-k) is mainstream already: Letta, mem0, Zep. This project goes after the parts that are still open:

- **Recall on implicit references.** "That bug from last week" shares few words or embedding neighbours with the memory it refers to.
- **Hybrid retrieval.** BM25 catches exact identifiers (`E1234`, `user_42`) that embeddings blur; vectors catch paraphrase. How to fuse them, measured.
- **Cost per turn.** Every recalled byte is paid for on every turn. Return less, keep accuracy.
- **Measured, not claimed.** LongMemEval accuracy and tokens per turn, compared against full-context, Letta and mem0 baselines.

Everything below the MCP layer (distance math, HNSW, BM25, fusion, storage) is implemented by hand, no index libraries.

## Crates

| Crate | Role |
|---|---|
| `crates/vecdb` | Vector index. Distance functions, exact `FlatIndex`, approximate `HnswIndex`, write-ahead log and snapshots. |
| `crates/lexical` | Keyword search. Tokenizer, inverted index, BM25 ranking. |
| `crates/memory` | Agent-memory layer. `MemoryStore` indexes each memory in both retrievers, fuses results (RRF first), filters by time and tag. |
| `crates/memory-mcp` | Binary. MCP server over stdio exposing `remember`, `recall`, `forget`. |
| `crates/bench` | Binary. ANN recall/latency sweeps, and the LongMemEval harness. |

Dependency direction: `vecdb`, `lexical` → `memory` → `memory-mcp`, `bench`. No external crates yet; add them deliberately.

## Working on it

```sh
cargo build                          # must always pass
cargo test --workspace               # stubs are ignored, so this passes
cargo test -p vecdb -- --ignored     # the tests you are trying to turn green
```

Remove a test's `#[ignore]` once you implement what it covers. CI runs fmt, clippy and the non-ignored tests.

## Roadmap

1. **Distances** (`vecdb::distance`). Scalar L2, dot, cosine. Then SIMD, and measure it.
2. **FlatIndex.** Exact k-NN with a top-k heap. This is ground truth for every recall number after it.
3. **BM25** (`lexical`). Tokenizer, inverted index, scoring.
4. **Fusion + MemoryStore** (`memory`). RRF, then `remember` / `recall` / `forget` on FlatIndex.
5. **MCP server** (`memory-mcp`). `initialize`, `tools/list`, `tools/call` over stdio. Wire it into Claude Code or another MCP client and use it daily.
6. **HNSW.** Paper Algorithms 1 to 5, deletes via tombstones. Target recall@10 ≥ 0.95 on 10k random vectors.
7. **ANN bench.** Recall vs. QPS curves on SIFT1M / GloVe, against FlatIndex.
8. **Storage.** WAL with checksums and torn-write recovery, then snapshots, then `mmap`.
9. **LongMemEval.** Accuracy per question type and tokens per turn vs. baselines.
10. **Research.** Implicit-reference recall, fusion variants, context compression. Write up the results.

## References

- Malkov & Yashunin, *Efficient and robust approximate nearest neighbor search using Hierarchical Navigable Small World graphs* (2016)
- Robertson & Zaragoza, *The Probabilistic Relevance Framework: BM25 and Beyond* (2009)
- Cormack, Clarke & Büttcher, *Reciprocal Rank Fusion outperforms Condorcet and individual rank learning methods* (SIGIR 2009)
- Wu et al., *LongMemEval: Benchmarking Chat Assistants on Long-Term Interactive Memory* (2024)
- [Model Context Protocol specification](https://modelcontextprotocol.io/specification)
- [ann-benchmarks](https://ann-benchmarks.com)
