# Topic 09: Error Handling

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What's the difference between `panic!` and returning a `Result`?**
   * *Key aspects to address:* Unrecoverable bugs/invariant violations (stack unwinding or process abort) vs expected recoverable operational failures (`Ok(T)` / `Err(E)` enum handling).

2. **What is the `?` operator, and how does it desugar?**
   * *Key aspects to address:* Early return syntax sugar, matching on `Result`/`Option`, implicit conversion via `From::from(err)` trait on error variants.

3. **What's the difference between `unwrap`, `expect`, and proper error propagation?**
   * *Key aspects to address:* Production code safety rules, panicking with default vs custom contextual failure messages, propagating errors upstream for caller handling.

4. **How would you design a custom error type? What role do `std::error::Error`, `thiserror`, and `anyhow` play?**
   * *Key aspects to address:* Custom enum error modeling, `Display` and `Source` implementations, static library errors (`thiserror`) vs application-level dynamic error contexts (`anyhow`).

5. **What's the difference between recoverable and unrecoverable errors in Rust's philosophy?**
   * *Key aspects to address:* Defensive programming, explicit API contract boundaries (`Result`), panics for contract violations/out of bounds, unwinding hooks (`std::panic::set_hook`).

6. **What is error type erasure, and when would you use `Box<dyn std::error::Error>`?**
   * *Key aspects to address:* Hiding concrete underlying error types, heterogeneous error handling in quick scripts / prototypes, dynamic dispatch overhead, losing concrete matching ability.
