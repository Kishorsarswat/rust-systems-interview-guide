# rust-systems-interview-guide
A comprehensive, hands-on guide for cracking advanced Rust engineering interviews.

This repository is built for backend developers and systems engineers aiming for senior-level roles at high-performance tech companies. Rust interviews are rarely just about trivia; they test your muscle memory with the borrow checker, your grasp of lock contention, and your ability to design scalable async architectures. Whether you are transitioning from a strong C++ background or scaling high-throughput distributed systems, this guide bridges the gap between theoretical knowledge and compilable code.

## 🏗️ Repository Structure
The repository is divided into theoretical question banks and hands-on Cargo workspaces:

- /concepts: Curated Markdown study guides covering everything from interior mutability to async task scheduling. Use these to structure mock interviews or test your verbal communication of technical tradeoffs.

- /live-coding/algorithms: A dedicated crate for pure logic, algorithmic data structures (e.g., tries, prefix sum maps), and dynamic programming.

- /live-coding/async-systems: A tokio-based crate focusing on backend patterns, channel backpressure, and concurrent state management (e.g., thread-safe rate limiters and lock-free concurrency).

- /cpp-to-rust: Quick-reference guides mapping manual memory management and C++ idioms to safe Rust abstractions.

## 🚀 Getting Started
The only way to truly learn Rust is to fight the compiler. The /live-coding directories are structured as isolated crates containing boilerplate setups and failing tests.

1. Clone the repository to your local machine.

2. Navigate to the specific workspace you want to practice (e.g., cd live-coding/algorithms).

3. Run the test suite: cargo test

4. Write the implementation to make the compiler happy and ensure all tests pass.

## 🤝 Contributing
This is a living repository designed to grow with the community. Pull requests are welcome to add new real-world interview questions, optimize existing solutions, or introduce new micro-benchmarks using Criterion. Please review CONTRIBUTING.md before submitting a PR.

How does this framing look to you? If you are happy with the README, should we move on to scaffolding the root Cargo.toml workspace, or would you prefer to format the first set of conceptual questions?

## License
This project is dual-licensed under the MIT and Apache 2.0 licenses.
