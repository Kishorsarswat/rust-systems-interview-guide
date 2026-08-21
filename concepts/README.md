# Rust Interview Questions — Conceptual Question Bank 🧠

A topic-organized question bank for Rust backend and systems engineering interview preparation. 

> **How to use this directory:**  
> These question guides are formatted for active recall and self-assessment during interview preparation, as well as for structuring mock technical interviews. Questions cover deep theoretical topics, runtime mechanics, borrow checker nuances, unsafe code invariants, and async system design.

---

## 📌 Master Topic Index

| # | Topic Guide | Key Focus Areas |
|---|-------------|------------------|
| 01 | [Ownership, Borrowing & Lifetimes](./01-ownership-borrowing-lifetimes.md) | Move vs Copy, borrow rules, elision, `'static`, `Cow<'a, T>` |
| 02 | [Smart Pointers & Interior Mutability](./02-smart-pointers-interior-mutability.md) | `Box`, `Rc`, `Arc`, `RefCell`, `Cell`, `Mutex`, `RwLock`, `Weak` |
| 03 | [Concurrency & Locks](./03-concurrency-and-locks.md) | Lock types, lock poisoning, atomics, memory orderings, cache line false sharing |
| 04 | [Async Rust & Futures](./04-async-rust-and-futures.md) | State machine desugaring, `Pin`, `Send`/`Sync`, cooperative scheduling, cancellation |
| 05 | [Concurrency Models: Threads vs Tasks](./05-concurrency-models-threads-vs-tasks.md) | OS threads vs Tokio tasks vs Rayon data parallelism, Actor pattern |
| 06 | [Channels](./06-channels.md) | `mpsc`, `broadcast`, `watch`, `oneshot`, bounded vs unbounded, backpressure |
| 07 | [Traits & Generics](./07-traits-and-generics.md) | Static vs dynamic dispatch, vtables, object safety, `Deref`, `From`/`Into`, newtype |
| 08 | [Macros](./08-macros.md) | Declarative (`macro_rules!`) vs procedural, macro hygiene, tradeoffs |
| 09 | [Error Handling](./09-error-handling.md) | `?` operator desugaring, custom errors, `thiserror`, `anyhow`, `Box<dyn Error>` |
| 10 | [Collections & Iterators](./10-collections-and-iterators.md) | `Vec` vs `VecDeque` vs `LinkedList`, Entry API, lazy iteration, performance |
| 11 | [Memory & Unsafe](./11-memory-and-unsafe.md) | `MaybeUninit`, raw pointers, undefined behavior invariants, self-referential structs |
| 12 | [Performance](./12-performance.md) | Micro-benchmarking, Criterion, stack vs heap, false sharing, zero-cost abstractions |
| 13 | [Testing & Tooling](./13-testing-and-tooling.md) | Unit vs integration testing, Cargo workspaces, Miri, sanitizers |
| 14 | [Web / Backend-Specific](./14-web-backend-specific.md) | Extractor pattern, `FromRequest`, compile-time SQL validation, streaming responses |
