# Rust Systems Interview Guide 🦀

A comprehensive, hands-on guide for cracking advanced Rust systems and backend engineering interviews.

This repository is built for backend developers, infrastructure engineers, and systems engineers aiming for senior-level roles at high-performance tech companies. Rust interviews test your muscle memory with the borrow checker, your grasp of lock contention, your understanding of memory layouts and FFI barriers, and your ability to design scalable async architectures.

---

## 📚 Table of Contents

- [🧠 Theoretical Question Bank](./concepts/README.md)
  - [01. Ownership, Borrowing & Lifetimes](./concepts/01-ownership-borrowing-lifetimes.md)
  - [02. Smart Pointers & Interior Mutability](./concepts/02-smart-pointers-interior-mutability.md)
  - [03. Concurrency & Locks](./concepts/03-concurrency-and-locks.md)
  - [04. Async Rust & Futures](./concepts/04-async-rust-and-futures.md)
  - [05. Concurrency Models: Threads vs Tasks](./concepts/05-concurrency-models-threads-vs-tasks.md)
  - [06. Channels](./concepts/06-channels.md)
  - [07. Traits & Generics](./concepts/07-traits-and-generics.md)
  - [08. Macros](./concepts/08-macros.md)
  - [09. Error Handling](./concepts/09-error-handling.md)
  - [10. Collections & Iterators](./concepts/10-collections-and-iterators.md)
  - [11. Memory & Unsafe](./concepts/11-memory-and-unsafe.md)
  - [12. Performance](./concepts/12-performance.md)
  - [13. Testing & Tooling](./concepts/13-testing-and-tooling.md)
  - [14. Web / Backend-Specific](./concepts/14-web-backend-specific.md)
- [🌐 Language Comparisons & FFI Guides](./ffi-and-comparisons/README.md)
  - [C Interop & FFI Barriers](./ffi-and-comparisons/c-and-ffi/README.md)
  - [C++ to Rust Systems Paradigms](./ffi-and-comparisons/cpp-to-rust/README.md)
- [💻 Live Coding Workspaces](./live-coding/README.md)
  - [Algorithms Workspace (`/live-coding/algorithms`)](./live-coding/algorithms/README.md)
  - [Async Systems Workspace (`/live-coding/async-systems`)](./live-coding/async-systems/README.md)

---

## 🏗️ Repository Structure

The repository is structured as a **Cargo Workspace** combining theoretical study guides with hands-on Rust crates:

- **/concepts**: Curated Markdown question bank covering 14 core topics. Use these to test yourself or structure technical interview screeners.
- **/ffi-and-comparisons**: Deep dives into C interop, `extern "C"`, FFI boundaries, memory layouts (`#[repr(C)]`), and C++ vs Rust paradigm mappings.
- **/live-coding/algorithms**: A dedicated crate for pure algorithmic data structures (e.g., $O(1)$ LRU Cache, Prefix Trie) with failing starter tests and reference solutions.
- **/live-coding/async-systems**: A Tokio-powered crate focusing on concurrent patterns, rate limiters, channels, worker pools, and non-blocking state management.

---

## 🚀 Quick Start

The best way to prepare is by writing compilable code. The `/live-coding` directories are structured as crates containing starter implementations and unit tests.

### 1. Test All Workspace Challenges
From the repository root, run:

```bash
# Run tests across all live coding crates
cargo test --workspace
```

### 2. Practice a Specific Challenge
Navigate to a workspace crate:

```bash
cd live-coding/algorithms
cargo test lru_cache
```

Failing tests will guide your implementation. If you get stuck or want to compare approach tradeoffs, idiomatic reference solutions are provided in `src/solutions/` inside each crate.

---

## 🤝 Contributing

Contributions are welcome! Whether adding new real-world interview questions, creating additional live-coding challenges, or optimizing benchmarks, please read [CONTRIBUTING.md](./CONTRIBUTING.md) for details.

---

## 📄 License

This project is dual-licensed under the [MIT License](./LICENSE-MIT) and [Apache 2.0 License](./LICENSE-APACHE).
