# Topic 13: Testing & Tooling

[← Back to Concepts Index](./README.md)

---

### Questions

1. **How do you write and organize unit tests vs integration tests in a Rust project?**
   * *Key aspects to address:* In-file module `#[cfg(test)] mod tests` (accessing private items) vs `tests/` directory root integration test suites (testing public API interface as external consumer).

2. **What is `cargo bench`, and how does it relate to Criterion?**
   * *Key aspects to address:* Built-in nightly benchmark framework (`#[bench]`) vs stable third-party statistical benchmarking library (`criterion-rs`).

3. **What tools would you use to detect data races or undefined behavior (e.g., Miri, sanitizers)?**
   * *Key aspects to address:* `cargo miri` (MID-level IR interpreter catching UB, alignment errors, stacked borrows violations at compile/test time), AddressSanitizer (ASan), ThreadSanitizer (TSan), `cargo-audit`.

4. **How do you structure a Rust workspace with multiple crates?**
   * *Key aspects to address:* Root `Cargo.toml` `[workspace]` declaration, workspace-level dependencies (`[workspace.dependencies]`), shared build profiles, binary vs library separation.
