# Live Coding: Async Systems & Concurrency 🚀

[← Back to Live Coding Index](../README.md)

This crate contains Tokio-based concurrent challenges focusing on real-world async backend architecture, rate limiting, channels, and worker pools.

---

## 📌 Challenges Included

| Challenge | File | Key Learning Objectives |
|-----------|------|-------------------------|
| **Async Rate Limiter** | [`src/rate_limiter.rs`](./src/rate_limiter.rs) | Token Bucket algorithm, atomic state updates, Tokio `Instant` time tracking, multi-threaded acquire |
| **Async Worker Pool** | [`src/worker_pool.rs`](./src/worker_pool.rs) | Tokio channels (`mpsc`, `oneshot`), task distribution, error propagation, graceful shutdown |

---

## 🚀 Running the Tests

```bash
# Run all async system tests
cargo test -p async-systems-live-coding

# Run specific challenge test
cargo test -p async-systems-live-coding rate_limiter
```
