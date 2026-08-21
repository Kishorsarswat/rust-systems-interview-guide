# Live Coding Practice Workspaces 💻

Hands-on Rust crates structured to simulate real-world live coding technical interviews.

---

## 🗂️ Available Workspaces

### 1. [Algorithms (`/live-coding/algorithms`)](./algorithms/README.md)
Focuses on pure data structures, memory indexing, and algorithmic problem solving:
- **LRU Cache**: Design and implement a Least Recently Used (LRU) Cache with $O(1)$ `get` and `put` ops without using third-party crates.
- **Prefix Trie**: Implement a memory-efficient Prefix Tree for fast string dictionary lookups.

### 2. [Async Systems (`/live-coding/async-systems`)](./async-systems/README.md)
Focuses on concurrent systems, Tokio channels, async tasks, and rate limiting:
- **Token Bucket Rate Limiter**: Thread-safe async rate limiter supporting concurrent token replenishment and non-blocking checks.
- **Bounded Worker Pool**: Task execution pool with Tokio channels (`mpsc`, `oneshot`) supporting graceful shutdown and cancellation handling.

---

## 🧪 How to Practice

1. Run tests inside a specific workspace:
   ```bash
   cd live-coding/algorithms
   cargo test
   ```
2. Read the problem prompt in the source module doc comments.
3. Replace the `todo!()` macros with your solution until all tests pass.
4. Reference idiomatic solutions in `src/solutions/` when finished!
