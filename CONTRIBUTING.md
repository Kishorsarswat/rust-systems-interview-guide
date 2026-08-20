# Contributing to `rust-systems-interview-guide`

First, thank you for taking the time to contribute! This repository grows through the shared knowledge of the backend and systems engineering community. 

Whether you are adding a tricky concurrency question from a recent interview, optimizing a live-coding solution, or fixing a typo, your help is appreciated.

## How Can You Contribute?

### 1. Adding Conceptual Questions (`/concepts`)
If you have encountered excellent interview questions regarding system design, memory safety, or advanced Rust mechanics:
* Submit a PR adding the question to the relevant Markdown file.
* Keep questions clear and open-ended. 
* Do not include the answers directly in the prompt; the goal is to encourage active recall and mock-interview discussion.

### 2. Adding Live Coding Challenges (`/live-coding`)
When adding new algorithmic or async system challenges:
* Place pure data structures and logic in `/live-coding/algorithms`.
* Place tokio-based, networking, and concurrency challenges in `/live-coding/async-systems`.
* Provide a boilerplate `.rs` file with the function signature and a standard `#[test]` module.
* Ensure the tests initially fail, requiring the user to implement the solution to pass.

### 3. C++ to Rust Translations (`/cpp-to-rust`)
To maintain our focus on high-performance systems engineering, all cross-language comparisons and architectural examples must utilize strictly **Rust** or **C++**. Please do not submit code examples or mappings in other languages.

## Coding Standards

To keep the repository clean and idiomatic, please adhere to the following before submitting a Pull Request:

* **Format your code:** Run `cargo fmt` on all Rust additions.
* **Lint your code:** Ensure `cargo clippy` passes without warnings. 
* **Comment the "Why":** In boilerplate code or test setups, comments should explain the constraints of the interview problem (e.g., "Note: Do not use standard library locking primitives for this task").

## Pull Request Process

1. Fork the repository and create your branch from `main`.
2. Name your branch descriptively (e.g., `add-gcra-rate-limiter-challenge`).
3. Ensure all existing workspace tests pass by running `cargo test --workspace`.
4. Open a Pull Request detailing what you added and the specific interview context it targets (if applicable).
