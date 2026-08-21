# Live Coding: Algorithms & Data Structures ⚡

[← Back to Live Coding Index](../README.md)

This crate contains classic low-level data structure and algorithmic challenges commonly asked in senior Rust systems interviews.

---

## 📌 Challenges Included

| Challenge | File | Key Learning Objectives |
|-----------|------|-------------------------|
| **LRU Cache** | [`src/lru_cache.rs`](./src/lru_cache.rs) | Index-based doubly-linked list, `HashMap`, constant-time $O(1)$ operations |
| **Prefix Trie** | [`src/trie.rs`](./src/trie.rs) | Recursive memory allocation, `HashMap` / array child nodes, string matching |

---

## 🚀 Running the Tests

```bash
# Run all algorithm tests
cargo test -p algorithms-live-coding

# Run specific challenge test
cargo test -p algorithms-live-coding lru_cache
```
